use std::{path::Path, fs::File, io::{self, Read, Write}};

use rayon::prelude::*;

use crate::matrix::{FloatMatrix, Matrix};

pub enum SolutionMethod {
    Direct,
    Iterative(usize)
}


/// Generates `chunk_size`x`chunk_size` key matrix. This key matrix later be divided into `L` and `U` (`key = L + U + I`)
/// and `K = (L + I)(U + I)` will give the encryption matrix with `det(K) = 1`.
pub fn gen_key(chunk_size: usize, integer: bool) -> FloatMatrix {
    // Generate `chunk_size`x`chunk_size` matrix, every element of which is between 0 and 1/`chunk_size`.
    let key = FloatMatrix::random(chunk_size, chunk_size, &0.0, &(1.0 / chunk_size as f32));
    
    // Make the diagonal of the key all ones.
    let key = key.clone() - key.hadamard(FloatMatrix::identity(chunk_size))
        + FloatMatrix::identity(chunk_size);
    
    // Round the elements of the key so that all entries are in 2-digit arithmetic.
    let key = key.round(chunk_size.ilog10() as usize + 1);
    
    // If requested so, convert the matrix to only have whole numbers.
    if integer {
        (key - Matrix::identity(chunk_size)).scale(chunk_size as f32 * 10.0).round(0) + Matrix::identity(chunk_size).scale(chunk_size as f32)
    } else {
        key
    }
}

/// Loads the key matrix from the given file.
pub fn load_key(file_path: &str) -> io::Result<(usize, FloatMatrix)> {
    let mut file = File::open(Path::new(file_path))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Read the first usize - the size of the key.
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

/// Stores the key matrix to the given file.
pub fn store_key(file_path: &str, key: &FloatMatrix) -> io::Result<()> {
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

/// Multiply `data` matrix by `(L+I)(U+I)` where `L + U + I = key`
pub fn encrypt_matrix_with_key(key: &FloatMatrix, data: &FloatMatrix) -> FloatMatrix {
    assert!(key.m == data.n);

    let k = key.tril().dot(&key.triu());

    // println!("Start multiplication...");
    // println!("k: {}x{}", k.n, k.m);
    // println!("data: {}x{}", data.n, data.m);
    
    let res = k.dot(&data);
    
    // println!("done");
    
    res
}

/// Decrypt the `data` matrix via a direct method. The method used is Thomas' algorithm (the best since I already have L and U stored).
pub fn dectrypt_matrix_with_key_direct(key: &FloatMatrix, data: FloatMatrix) -> FloatMatrix {
    let l = key.tril();
    let u = key.triu();
    
    let data = data.transpose();

    // println!("Start direct method...");
    // println!("\tk: {}x{}", key.n, key.m);
    // println!("\tdata: {}x{}", data.n, data.m);
    
    // The decrypted encrypted input matrix.
    let decrypted = FloatMatrix::from(
        (0..data.n)
            .into_par_iter()
            .map(|i| {
                FloatMatrix::solve_system_lu(&l, &u, data.data[i].clone())
            })
            .collect::<Vec<_>>(),
    )
    .transpose();
    
    // println!("done");

    decrypted
}

/// Decrypt the `data` matrix via an iterative method. The method used is SOR (Successive Over-Relaxation) with `omega = 1.3`
pub fn dectrypt_matrix_with_key_iterative(key: &FloatMatrix, data: FloatMatrix, iterations: usize) -> FloatMatrix {
    let k = key.tril().dot(&key.triu());

    let data = data.transpose();
    
    // println!("Start iterarive method...");
    // println!("\tk: {}x{}", key.n, key.m);
    // println!("\tdata: {}x{}", data.n, data.m);
    
    // The decrypted encrypted input matrix.
    let decrypted = FloatMatrix::from(
        (0..data.n)
            .into_par_iter()
            .map(|i| {
                FloatMatrix::solve_system_iterative(&k, data.data[i].clone(), 1.3, iterations)
            })
            .collect::<Vec<_>>(),
    )
    .transpose();
    
    // println!("done.");
    
    decrypted
}