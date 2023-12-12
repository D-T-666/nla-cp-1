use std::{fs::File, io, path::Path};

use wav::{BitDepth, Header};

// Encryption:
// 1. reade the file
// 2. convert to 2-digit (decimal) arithmetic
//    - split the data into nibbles (4-bit integers)
// 3. encrypt with the key matrix
// 4. split the resulting 32-bit numbers into 16-bit numbers
// 5. store the resulting integers into an encrypted audio file

// Decryption:
// 1. read the file
// 2. glue pairs of 16-bit integers into 32-bit floats
// 3. decrypt with the key matrix
// 4. round the floats to the nearest 4-bit integers
// 5. glue the quadruples of 4-bit integers back to 16-bit integes
// 6. store the resulting integers into a decrypted audio file

use crate::{
    dectrypt_matrix_with_key_direct, dectrypt_matrix_with_key_iterative, encrypt_matrix_with_key,
    matrix::FloatMatrix, text::{matrix_to_vector, vector_to_matrix},
};

pub struct AudioContents {
    header: Header,
    data: Vec<i16>,
}

/// ## Read audio file contents
/// Reads the contents of an audio file to `AudioContents` struct.
/// ### Limiatations:
/// - can only read 16 bit `.wav` files.
pub fn read_audio_file_contents(file_path: &str) -> io::Result<AudioContents> {
    let mut file = File::open(Path::new(file_path))?;
    let (header, data) = wav::read(&mut file)?;
    let BitDepth::Sixteen(data) = data else {
        panic!("The file is not 16 bit.")
    };
    Ok(AudioContents { header, data })
}

pub struct AudioMatrix {
    matrix: FloatMatrix,
    length: usize,
}

/// ## Audio file to matrix
/// Converts audio file contents to matrix form.
/// 
pub fn audio_file_to_matrix(audio: &AudioContents, chunk_size: usize) -> AudioMatrix {
    // Chop the 16 bit numbers into four 4 bit numbers.
    // This is the conversion to 2-digit arithmetic.
    let mut two_digit = audio
        .data
        .iter()
        .flat_map(|&num| {
            [
                ((num >> 12) & 0xF) as u8,
                ((num >> 8) & 0xF) as u8,
                ((num >> 4) & 0xF) as u8,
                ((num >> 0) & 0xF) as u8,
            ]
        })
        .collect::<Vec<_>>();

    let data_length = two_digit.len();

    // Pad the data if necessary.
    if data_length % chunk_size != 0 {
        two_digit.append(&mut vec![0; chunk_size - (data_length % chunk_size)])
    };

    let res = (0..two_digit.len())
        .step_by(chunk_size)
        .map(|i| {
            two_digit[i..i + chunk_size]
                .iter()
                .map(|&c| c as f32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    AudioMatrix {
        matrix: FloatMatrix::from(res).transpose(),
        length: data_length,
    }
}

fn write_audio_file_vector(
    file_path: &str,
    file_contents: &AudioContents,
    data: Vec<i16>,
    header: Option<usize>,
) -> io::Result<()> {
    let mut file = File::create(Path::new(file_path)).unwrap();

    let mut data = data;
    
    if let Some(header) = header {
        for i in (0..64).step_by(16) {
            data.push(((header >> i) & 0xFFFF) as i16);
        }
    }

    wav::write(file_contents.header, &BitDepth::Sixteen(data), &mut file)?;

    Ok(())
}

pub fn encrypt_audio_with_key(file_path: &str, key: &FloatMatrix) -> io::Result<()> {
    let audio = read_audio_file_contents(file_path)?;
    
    let two_digit = audio
        .data
        .iter()
        .flat_map(|&num| {
            [
                ((num >> 12) & 0xF) as usize,
                ((num >> 8) & 0xF) as usize,
                ((num >> 4) & 0xF) as usize,
                ((num >> 0) & 0xF) as usize,
            ]
        })
        .collect::<Vec<_>>();
    println!("2-digit:\n{:?}", &two_digit[..100]);
        
    let data = vector_to_matrix(two_digit, key.n, |x| x as f32);
    // println!("data:\n{}", data);

    let encrypted = encrypt_matrix_with_key(key, &data);
    println!("floats:\n{:?}", &encrypted.transpose().data[0][..100]);
    
    let vec = matrix_to_vector(encrypted.transpose(), |x| x.to_bits() as usize);
    println!("bits of floats:\n{:?}", &vec[..100]);
    
    let vec = vec.iter().flat_map(|&x| [
        ((x >> 16) & 0xFFFF) as i16,
        (x & 0xFFFF) as i16,
    ]).collect::<Vec<_>>();
    println!("slplit:\n{:?}", &vec[..100]);

    let header = Some(vec.len());

    write_audio_file_vector(
        format!("{}-encrypted.wav", file_path.strip_suffix(".wav").unwrap()).as_str(),
        &audio,
        vec,
        header,
    )?;

    Ok(())
}



// Decryption:
// 1. read the file
// 2. glue pairs of 16-bit integers into 32-bit floats
// 3. convert the vector to a matrix
// 4. decrypt with the key matrix
// 5. round the floats to the nearest 4-bit integers
// 6. convert to a vector
// 7. glue the quadruples of 4-bit integers back to 16-bit integes
// 8. store the resulting integers into a decrypted audio file

pub fn decrypt_audio_with_key(file_path: &str, key: &FloatMatrix, direct: bool) -> io::Result<()> {
    // 1. read the file
    let audio = read_audio_file_contents(file_path)?;
    
    // 2. glue pairs of 16-bit integers into 32-bit floats
    let vec = (0..audio.data.len()).step_by(2).map(|i| {
        ((audio.data[i] as u16 as usize) << 16) | (audio.data[i + 1] as u16 as usize)
    }).collect::<Vec<_>>();
    println!("32 bit float vector:\n{:?}", &vec[..100]);
    
    // 3. convert the vector to a matrix
    let data = vector_to_matrix(
        vec,//[8..].to_vec(),
        key.n,
        |x| f32::from_bits(x as u32)
    );
    // Can't print the matrix since it's veeeeery large.
    // println!("floats:\n{:?}", &data.transpose().data[0][..100]);
    
    // 4. decrypt with the key matrix
    let decrypted = if direct {
        println!("using direct method.");
        dectrypt_matrix_with_key_direct(key, data)
    } else {
        println!("using iterative method.");
        dectrypt_matrix_with_key_iterative(key, data)
    };
    // Very large matrix here aswell.
    
    // 5. round the floats to the nearest 4-bit integers
    let decrypted = decrypted.transpose().round(0);
    // println!("matrix:\n{decrypted}");
    
    // 6. convert to a vector
    let vec = matrix_to_vector(decrypted, |x| x as u8 as usize);
    println!("2-digit arithmetic decrypted:\n{:?}", &vec[..100]);    
    
    // 7. glue the quadruples of 4-bit integers back to 16-bit integes
    let sixteen = (0..vec.len()).step_by(4).map(|i| {
        ((vec[i + 0] << 12) |
        (vec[i + 1] << 8) |
        (vec[i + 2] << 4) |
        (vec[i + 3] << 0)) as i16
    }).collect::<Vec<_>>();
    println!("glued:\n{:?}", &sixteen[..25]);    
    
    // 8. store the resulting integers into a decrypted audio file
    write_audio_file_vector(
        format!("{}-decrypted.wav", file_path.strip_suffix(".wav").unwrap()).as_str(),
        &audio,
        sixteen,
        None,
    )?;

    Ok(())
}
