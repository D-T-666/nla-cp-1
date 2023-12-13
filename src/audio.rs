use std::{fs::File, io, path::Path};

use wav::{BitDepth, Header};

use crate::{
    encryption::{
        dectrypt_matrix_with_key_direct, dectrypt_matrix_with_key_iterative,
        encrypt_matrix_with_key,
    },
    matrix::FloatMatrix,
    text::{matrix_to_vector, vector_to_matrix},
    SolutionMethod,
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

/// Writes the given 16-bit vector of integers into a `.wav` file.
fn write_audio_file_vector(
    file_path: &str,
    file_contents: &AudioContents,
    data: Vec<i16>,
    header: Option<usize>,
) -> io::Result<()> {
    let mut file = File::create(Path::new(file_path)).unwrap();

    let mut data = data;

    // If a header is provided, chop it up into 16 bit integers and preppend it to the stream.
    if let Some(header) = header {
        data = [
            vec![((header >> 16) & 0xFFFF) as i16, (header & 0xFFFF) as i16],
            data,
        ]
        .concat()
    }

    wav::write(file_contents.header, &BitDepth::Sixteen(data), &mut file)?;

    Ok(())
}

/// ### Encryption:
/// 1. reade the file
/// 2. convert to 2-digit (decimal) arithmetic
///    - split the data into nibbles (4-bit integers)
/// 3. encrypt with the key matrix
/// 4. split the resulting 32-bit numbers into 16-bit numbers
/// 5. store the resulting integers into an encrypted audio file
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
    let data_length = two_digit.len();
    // println!("2-digit:\n{:?}", &two_digit[..100]);

    let data = vector_to_matrix(two_digit, key.n, |x| x as f32);
    // println!("data:\n{}", data);

    let encrypted = encrypt_matrix_with_key(key, &data);
    // println!("floats:\n{:?}", &encrypted.transpose().data[0][..100]);

    let vec = matrix_to_vector(encrypted.transpose(), |x| x.to_bits() as usize);
    // println!("bits of floats:\n{:?}", &vec[..100]);

    let data = vec
        .iter()
        .flat_map(|&x| [((x >> 16) & 0xFFFF) as i16, (x & 0xFFFF) as i16])
        .collect::<Vec<_>>();
    // println!("slplit:\n{:?}", &vec[..100]);

    write_audio_file_vector(
        format!("{}-encrypted.wav", file_path.strip_suffix(".wav").unwrap()).as_str(),
        &audio,
        data,
        Some(data_length),
    )?;

    Ok(())
}

/// ### Decryption:
/// 1. read the file
/// 2. glue pairs of 16-bit integers into 32-bit floats
/// 3. convert the vector to a matrix
/// 4. decrypt with the key matrix
/// 5. round the floats to the nearest 4-bit integers
/// 6. convert to a vector
/// 7. glue the quadruples of 4-bit integers back to 16-bit integes
/// 8. store the resulting integers into a decrypted audio file
pub fn decrypt_audio_with_key(
    file_path: &str,
    key: &FloatMatrix,
    method: SolutionMethod,
) -> io::Result<()> {
    // 1. read the file
    let audio = read_audio_file_contents(file_path)?;

    // 2. glue pairs of 16-bit integers into 32-bit floats
    let float_bits = (0..audio.data.len())
        .step_by(2)
        .map(|i| ((audio.data[i] as u16 as usize) << 16) | (audio.data[i + 1] as u16 as usize))
        .collect::<Vec<_>>();
    let data_length = float_bits[0];
    // println!("32 bit float vector:\n{:?}", &vec[..100]);

    // 3. convert the vector to a matrix
    let data = vector_to_matrix(float_bits[1..].to_vec(), key.n, |x| f32::from_bits(x as u32));
    // Can't print the matrix since it's veeeeery large.
    // println!("floats:\n{:?}", &data.transpose().data[0][..100]);

    // 4. decrypt with the key matrix
    let decrypted = if let SolutionMethod::Iterative(iterations) = method {
        dectrypt_matrix_with_key_iterative(key, data, iterations)
    } else {
        dectrypt_matrix_with_key_direct(key, data)
    };
    // Very large matrix here aswell.

    // 5. round the floats to the nearest 4-bit integers
    let decrypted = decrypted.transpose().round(0);
    // println!("matrix:\n{decrypted}");

    // 6. convert to a vector
    let raw_vec = matrix_to_vector(decrypted, |x| x as u8 as usize);
    let truncated_vec = raw_vec[..data_length].to_vec();
    // println!("2-digit arithmetic decrypted:\n{:?}", &vec[..100]);

    // 7. glue the quadruples of 4-bit integers back to 16-bit integes
    let sixteen = (0..truncated_vec.len())
        .step_by(4)
        .map(|i| {
            ((truncated_vec[i + 0] << 12) | (truncated_vec[i + 1] << 8) | (truncated_vec[i + 2] << 4) | (truncated_vec[i + 3] << 0)) as i16
        })
        .collect::<Vec<_>>();
    // println!("glued:\n{:?}", &sixteen[..25]);

    // 8. store the resulting integers into a decrypted audio file
    write_audio_file_vector(
        format!("{}-decrypted.wav", file_path.strip_suffix(".wav").unwrap()).as_str(),
        &audio,
        sixteen,
        None,
    )?;

    Ok(())
}
