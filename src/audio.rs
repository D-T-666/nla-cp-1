use std::{fs::File, io, path::Path};

use wav::{BitDepth, Header};

use crate::matrix::FloatMatrix;

fn read_audio_file_vector(file_path: &str, chunk_size: usize) -> io::Result<(FloatMatrix, Header, usize)> {
    let mut file = File::open(Path::new(file_path)).unwrap();
    let (header, data) = wav::read(&mut file).unwrap();

    let BitDepth::Sixteen(data) = data else {
        panic!()
    };
    let data_len = data.len();

    let mut two_digit = Vec::new();

    for num in data {
        two_digit.push((num >> 0) & 15);
        two_digit.push((num >> 4) & 15);
        two_digit.push((num >> 8) & 15);
        two_digit.push((num >> 12) & 15);
    }
    let two_digit_len = two_digit.len();

    let data_padded = if two_digit_len % chunk_size == 0 {
        two_digit
    } else {
        [
            two_digit,
            vec![0; chunk_size - (two_digit_len % chunk_size)],
        ]
        .concat()
    };

    let res = (0..data_padded.len())
        .step_by(chunk_size)
        .map(|i| {
            data_padded[i..i + chunk_size]
                .iter()
                .map(|&c| c as f32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok((FloatMatrix::from(res).transpose(), header, data_len))
}

fn write_audio_file_vector(file_path: &str, header: Header, data_len: usize) -> io::Result<()> {
    let mut file = File::open(Path::new(file_path)).unwrap();
    
    // let 
    
    wav::write(header, &track, &mut file)?;
    
    Ok(())
}
