mod audio;
mod matrix;
mod strat;
mod text;

use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use clap::{Parser, Subcommand};
use matrix::FloatMatrix;
use text::{decrypt_text_with_key, encrypt_text_with_key};

use crate::audio::{decrypt_audio_with_key, encrypt_audio_with_key};

fn gen_key(chunk_size: usize) -> FloatMatrix {
    let l = FloatMatrix::random(chunk_size, chunk_size, &0.0, &1.0).scale(1.0 / chunk_size as f32);
    let l = l.clone() - l.hadamard(FloatMatrix::identity(chunk_size))
        + FloatMatrix::identity(chunk_size);
        
    l.round(chunk_size.ilog10() as usize + 1)
}

fn load_key(file_path: &str) -> io::Result<(usize, FloatMatrix)> {
    let mut file = File::open(Path::new(file_path))?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut n_bytes = [0; 8];
    for i in 0..8 {
        n_bytes[i] = buf[i];
    }
    let n = usize::from_be_bytes(n_bytes);

    let mut key = FloatMatrix::zero(n, n);

    for i in 0..n {
        for j in 0..n {
            let ind = (i * n + j + 2) * 4;

            let mut bytes = [0; 4];
            for k in 0..4 {
                bytes[k] = buf[k + ind];
            }
            key.data[i][j] = f32::from_be_bytes(bytes);
        }
    }

    Ok((n, key))
}

fn store_key(file_path: &str, key: &FloatMatrix) -> io::Result<()> {
    let mut file = File::create(Path::new(file_path))?;

    let header = key.n.to_be_bytes();
    let body = key
        .data
        .iter()
        .flat_map(|row| row.iter().map(|elt| elt.to_be_bytes()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .concat();

    let data = [header.to_vec(), body].concat();

    file.write(data.as_slice())?;

    Ok(())
}

fn encrypt_matrix_with_key(key: &FloatMatrix, data: &FloatMatrix) -> FloatMatrix {
    assert!(key.m == data.n);

    let k = key.tril().dot(&key.triu());

    println!("Start multiplication...");
    let res = k.dot(&data);
    println!("done");
    res
}

fn dectrypt_matrix_with_key_direct(key: &FloatMatrix, data: FloatMatrix) -> FloatMatrix {
    let l = key.tril();
    let u = key.triu();

    // The decoded encoded input matrix.
    let decoded = FloatMatrix::from(
        (0..data.m)
            .map(|i| {
                let d = FloatMatrix::from(vec![data.transpose()[i].clone()]).transpose();
                FloatMatrix::solve_system(&l, &u, &d).transpose()[0].clone()
            })
            .collect::<Vec<_>>(),
    )
    .transpose();

    decoded
}

fn dectrypt_matrix_with_key_iterative(key: &FloatMatrix, data: FloatMatrix) -> FloatMatrix {
    let k = key.tril().dot(&key.triu());

    // The decoded encoded input matrix.
    let decoded = FloatMatrix::from(
        (0..data.m)
            .map(|i| {
                let d = FloatMatrix::from(vec![data.transpose()[i].clone()]).transpose();
                FloatMatrix::solve_system_iterative(&k, &d, 1.2, 5).transpose()[0].clone()
            })
            .collect::<Vec<_>>(),
    )
    .transpose();
    decoded.round(0)
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenKey {
        #[arg(short, long)]
        key_path: String,
        
        #[arg(short, long)]
        chunk_size: usize,
    },
    Encrypt {
        #[arg(short, long)]
        key_path: String,
        
        #[arg(short, long)]
        file_path: String,
    },
    Decrypt {
        #[arg(short, long)]
        key_path: String,
        
        #[arg(short, long)]
        file_path: String,
        
        #[arg(short, long)]
        direct: bool,
    },
}

fn main() {
    let Args { cmd } = Args::parse();

    match cmd {
        Commands::GenKey {
            key_path,
            chunk_size,
        } => {
            let key = gen_key(chunk_size);
            if chunk_size <= 30 {
                println!("{}", key);
                println!("\n{}", key.tril().dot(&key.triu()).round(chunk_size.ilog10() as usize + 2));
            }
            store_key(key_path.as_str(), &key).unwrap();
        }
        Commands::Encrypt {
            key_path,
            file_path,
        } => {
            let (_, key) = load_key(key_path.as_str()).unwrap();

            if file_path.ends_with(".txt") {
                encrypt_text_with_key(file_path.as_str(), &key).unwrap();
            } else if file_path.ends_with(".wav") {
                encrypt_audio_with_key(file_path.as_str(), &key).unwrap();
            } else {
                panic!("Invalid file type!");
            }
        }
        Commands::Decrypt {
            key_path,
            file_path,
            direct,
        } => {
            let (_, key) = load_key(key_path.as_str()).unwrap();

            if file_path.ends_with(".txt") {
                decrypt_text_with_key(file_path.as_str(), &key, direct).unwrap();
            } else if file_path.ends_with(".wav") {
                decrypt_audio_with_key(file_path.as_str(), &key, direct).unwrap();
            } else {
                panic!("Invalid file type!");
            }
        }
    }
}
