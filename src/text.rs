use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use crate::{
    encryption::{
        dectrypt_matrix_with_key_direct, dectrypt_matrix_with_key_iterative,
        encrypt_matrix_with_key,
    },
    matrix::FloatMatrix,
    SolutionMethod,
};

/// Encrypts the given text file with the given key.
pub fn encrypt_text_with_key(file_path: &str, key: &FloatMatrix) -> io::Result<()> {
    // Convert the ASCII codes to 2-digit arithmetic by
    // chopping it up into nibbles.
    let vec = read_two_digit_text(file_path)?;
    // println!("Input data (in two digit arithmetic):\n{vec:?}");

    // Convert the vector of integers into a matrix with floating point entries
    let data = vector_to_matrix(vec.iter().map(|&x| x as usize).collect(), key.n, |x| {
        x as f32
    });
    // println!("converted to matrix form:\n{data}");

    let encrypted = encrypt_matrix_with_key(key, &data);
    // println!("encrypted:\n{encrypted}");

    let encrypted_vector = matrix_to_vector(encrypted.transpose(), |x| x.to_bits() as usize);
    // println!("vectorized:\n{encrypted:?}");
    
    // Split the floating point numbers' binary representations into 4-bit pieces
    let data = encrypted_vector
        .iter()
        .flat_map(|&x| {
            (0..32)
                .step_by(4)
                .rev()
                .map(|i| ((x >> i) & 0xF) as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // println!("encrypted data:\n{:?}", data);

    // Shift all numbers by the ASCII code of 'a' so that all numbers, when converted to
    // chars end up in the lowercase alphabet.
    let data = data.iter().map(|x| x + 'a' as u8).collect();

    write_text_file_vector(
        format!("{}-encrypted.txt", file_path.strip_suffix(".txt").unwrap()).as_str(),
        data,
        Some(vec.len().to_string()),
    )?;

    Ok(())
}

/// Decrypts the given text file with the given key.
pub fn decrypt_text_with_key(
    file_path: &str,
    key: &FloatMatrix,
    method: SolutionMethod,
) -> io::Result<()> {
    let s = read_text_file_contents(file_path)?;
    // Read the header number.
    let text_length = s[..s.find(" ").unwrap()].parse::<usize>().unwrap();
    // Truncate the string to remove the header.
    let s = &s[s.find(" ").unwrap() + 1..];

    // Shift all numbers by the ASCII code of 'a' back
    let vec = string_to_vector(s)
        .iter()
        .map(|&x| x - 'a' as usize)
        .collect::<Vec<_>>();

    // println!("v{vec:?}");

    // Restore the floats from the 4-bit pieces
    let float_bits = (0..vec.len())
        .step_by(8)
        .map(|i| {
            ((vec[i] as usize) << 28)
                + ((vec[i + 1] as usize) << 24)
                + ((vec[i + 2] as usize) << 20)
                + ((vec[i + 3] as usize) << 16)
                + ((vec[i + 4] as usize) << 12)
                + ((vec[i + 5] as usize) << 8)
                + ((vec[i + 6] as usize) << 4)
                + (vec[i + 7] as usize)
        })
        .collect();
    // println!("v{float_bits:?}");

    let data = vector_to_matrix(float_bits, key.n, |x| f32::from_bits(x as u32));
    // println!("floated:\n{data}");

    let decrypted = if let SolutionMethod::Iterative(iterations) = method {
        dectrypt_matrix_with_key_iterative(key, data, iterations)
    } else {
        dectrypt_matrix_with_key_direct(key, data)
    };
    // println!("decrypted:\n{decrypted}");

    let vec = matrix_to_vector(decrypted.transpose().round(0).clone(), |x| x as usize)
        [..text_length]
        .to_vec();
    // println!("vec:\n{:?}", vec);

    // Restore the ASCII codes form the 4-bites .
    let vec = (0..vec.len())
        .step_by(2)
        .map(|i| ((vec[i] << 4) | vec[i + 1]) as u8)
        .collect::<Vec<_>>();
    // println!("d:{vec:?}");

    write_text_file_vector(
        format!("{}-decrypted.txt", file_path.strip_suffix(".txt").unwrap()).as_str(),
        vec,
        None,
    )?;

    Ok(())
}

/// Returns the contents of a file as a `String`.
pub fn read_text_file_contents(file_path: &str) -> io::Result<String> {
    let mut file = File::open(Path::new(file_path))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

/// Converts a string to a vector of integers of corresponding ASCII codes.
fn string_to_vector(s: &str) -> Vec<usize> {
    s.chars().map(|c| c as usize).collect::<Vec<_>>()
}

fn read_two_digit_text(file_path: &str) -> io::Result<Vec<u8>> {
    Ok(
        string_to_vector(read_text_file_contents(file_path)?.as_str())
            .iter()
            .flat_map(|&x| [((x >> 4) & 0xF) as u8, ((x >> 0) & 0xF) as u8])
            .collect::<Vec<_>>(),
    )
}

/// Converts `Vec<usize>` to a matrix with `chunk_size` rows
pub fn vector_to_matrix(
    vec: Vec<usize>,
    chunk_size: usize,
    convertor: fn(usize) -> f32,
) -> FloatMatrix {
    let length = vec.len();

    let mut vec = vec;
    if length % chunk_size != 0 {
        vec.append(&mut vec![
            vec[length - 1];
            chunk_size - (length % chunk_size)
        ]);
    }

    // The vector of vectors of the char codes of the padded input string.
    FloatMatrix::from(
        (0..vec.len())
            .step_by(chunk_size)
            .map(|i| {
                vec[i..i + chunk_size]
                    .iter()
                    .map(|&c| convertor(c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .transpose()
}

/// Converts a matrix to a vector while applying a `convertor` function to every element.
pub fn matrix_to_vector(matrix: FloatMatrix, convertor: fn(f32) -> usize) -> Vec<usize> {
    matrix
        .data
        .iter()
        .flat_map(|x| x.iter().map(|&e| convertor(e)).collect::<Vec<_>>())
        .collect()
}

/// Writes the contents of a byte vector to the file as ASCII Symbols
pub fn write_text_file_vector(
    file_path: &str,
    data: Vec<u8>,
    header: Option<String>,
) -> io::Result<()> {
    let mut file = File::create(Path::new(file_path))?;

    let out = data.iter().map(|&c| c as char).collect::<String>();

    // println!("{}", &out[..data_length]);

    if let Some(header) = header {
        file.write(format!("{} {}", header, out).as_bytes())?
    } else {
        file.write(out.as_bytes())?
    };

    Ok(())
}
