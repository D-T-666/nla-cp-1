use std::{fs::File, path::Path, io::{Read, Write, self}};

use crate::{matrix::FloatMatrix, encrypt_matrix_with_key, dectrypt_matrix_with_key};

/// Returns the contents of a file as a `String`
fn read_text_file_contents(file_path: &str) -> io::Result<String> {
    let mut file = File::open(Path::new(file_path))?;
    
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    
    Ok(buf)
}

/// Converts `s` to a matrix with `chunk_size` columns
fn string_to_matrix(s: String, chunk_size: usize) -> (FloatMatrix, usize) {
    let s_len = s.len();
    
    // The input string, but padded.
    let s_padded = if s.len() % chunk_size == 0 {
        String::from(s)
    } else {
        format!("{s}{}", " ".repeat(chunk_size - (s.len() % chunk_size)))
    };

    // The vector of vectors of the char codes of the padded input string.
    (FloatMatrix::from((0..s_padded.len())
        .step_by(chunk_size)
        .map(|i| {
            s_padded[i..i + chunk_size]
                .chars()
                .map(|c| c as u32 as f32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()), s_len)
}

fn write_text_file_vector(file_path: &str, data: FloatMatrix, header: String, data_length: usize) -> io::Result<()> {
    let mut file = File::open(Path::new(file_path))?;
    
    let out = data
        .transpose()
        .data
        .iter()
        .map(|row| {
            row.iter()
                .map(|&c| c as i32 as u8 as char)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("");
        
    file.write(format!("{}{}", header, &out[..data_length]).as_bytes())?;
    
    Ok(())
}

fn encrypt_string_with_key(file_path: &str, key: FloatMatrix) -> io::Result<()> {
    let s = read_text_file_contents(file_path)?;
    let (data, text_length) = string_to_matrix(s, key.n);
    
    let encrypted = encrypt_matrix_with_key(key, data);
    
    let enc_length = text_length + key.n - ((text_length - 1) % key.n + 1);
    
    write_text_file_vector(file_path, encrypted, format!("{}", text_length), enc_length)?;
    
    Ok(())
}

fn decrypt_string_with_key(file_path: &str, key: FloatMatrix) -> io::Result<()> {
    let s = read_text_file_contents(file_path)?;
    let (data, text_length) = string_to_matrix(s, key.n);
    
    let encrypted = dectrypt_matrix_with_key(key, data);
    
    let text_length = s[..s.find(" ").unwrap()].parse::<usize>().unwrap();
    
    write_text_file_vector(file_path, encrypted, String::new(), text_length)?;
    
    Ok(())
}