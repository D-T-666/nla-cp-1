mod matrix;
mod strat;
mod text;
mod audio;

use std::{fs::File, path::Path, io::{self, Write, Read}};

use matrix::FloatMatrix;
use wav::BitDepth;

fn gen_key(chunk_size: usize) -> FloatMatrix {
    let l = FloatMatrix::random(chunk_size, chunk_size, &-1.0, &1.0)
        .tril()
        .scale(1.0 / chunk_size as f32);
    let mut l = l.clone() - l.hadamard(FloatMatrix::identity(chunk_size)) + FloatMatrix::identity(chunk_size);
    l.round(1);

    let u = FloatMatrix::random(chunk_size, chunk_size, &-1.0, &1.0)
        .triu()
        .scale(1.0 / chunk_size as f32);
    let mut u = u.clone() - u.hadamard(FloatMatrix::identity(chunk_size));
    u.round(1);
    
    l + u
}

fn load_key(file_path: &str) -> io::Result<FloatMatrix> {
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
            let ind = (i * n + j) * 4 + 8;
            
            let mut bytes = [0; 4];
            for k in 0..4 {
                bytes[k] = buf[k + ind];
            }
            key.data[i][j] = f32::from_be_bytes(bytes);
        }
    }
    
    Ok(key)
}

fn store_key(file_path: &str, key: FloatMatrix) -> io::Result<()> {
    let mut file = File::create(Path::new(file_path))?;
    
    let header = key.n.to_be_bytes();
    let body = key.data.iter().flat_map(|row| {
        row.iter().map(|elt| {
            elt.to_be_bytes()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>().concat();
    
    let data = [header.to_vec(), body].concat();
    
    file.write(data.as_slice())?;
    
    Ok(())
}

fn encrypt_matrix_with_key(key: FloatMatrix, data: FloatMatrix) -> FloatMatrix {
    assert!(key.m == data.n);
    
    let k = key.tril().clone().dot(&key.triu());    
    
    let mut encoded = k.dot(&data);
    encoded.round(0);
    
    encoded
}

fn dectrypt_matrix_with_key(key: FloatMatrix, data: FloatMatrix) -> FloatMatrix {
    let l = key.tril();
    let u = key.triu();

    // The decoded encoded input matrix.
    let mut decoded = FloatMatrix::from(
        (0..data.m)
            .map(|i| {
                let d = FloatMatrix::from(vec![data.transpose()[i].clone()]).transpose();
                FloatMatrix::solve_system(&l, &u, &d).transpose()[0].clone()
            })
            .collect::<Vec<_>>(),
    )
    .transpose();
    decoded.round(0);
    
    decoded
}



// fn encrypt_text(path: &str, chunk_size: usize) {
//     let mut buf = String::new();
//     File::open(Path::new(path)).unwrap()
//         .read_to_string(&mut buf).unwrap();
//     let inp = buf.as_str();
//     let inp_len = inp.len();
    
//     // The input string, but padded.
//     let inp_padded = if inp.len() % chunk_size == 0 {
//         String::from(inp)
//     } else {
//         format!("{inp}{}", " ".repeat(chunk_size - (inp.len() % chunk_size)))
//     };

//     // The vector of vectors of the char codes of the padded input string.
//     (0..inp_padded.len())
//         .step_by(chunk_size)
//         .map(|i| {
//             inp_padded[i..i + chunk_size]
//                 .chars()
//                 .map(|c| c as i64 as f64)
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
        
//     // The encoded input matrix.
//     let mut encoded = k.dot(&d);
//     encoded.round(1);

//     // The decoded encoded input matrix.
//     let mut decoded = FloatMatrix::from(
//         (0..encoded.m)
//             .map(|i| {
//                 let d = FloatMatrix::from(vec![encoded.transpose()[i].clone()]).transpose();
//                 FloatMatrix::solve_system(&l, &u, &d).transpose()[0].clone()
//             })
//             .collect::<Vec<_>>(),
//     )
//     .transpose();
//     decoded.round(0);
// }

// fn main() {
//     let chunk_size = 10;
//     let key = gen_key(chunk_size);
    
//     store_key("test.key", key.clone()).unwrap();
//     let loaded_key = load_key("test.key").unwrap();
    
//     println!("{}", key == loaded_key);
// }

fn main() {
    let chunk_size = 600;

    let lu = gen_key(chunk_size);

    let k = lu.tril().clone().dot(&lu.triu());
    println!("{}", line!());

    // println!("{}", l);
    // println!("{}", u);
    // println!("{}", k);
    // I am your father!
    // - NOOOOO
    
    let inp_len;
    let mut optional_header = None;

    let d = FloatMatrix::from(if false {
        // The input string.
        let inp = "Gamarjoba, chemo mayurebelo!";
        inp_len = inp.len();
        
        // The input string, but padded.
        let inp_padded = if inp.len() % chunk_size == 0 {
            String::from(inp)
        } else {
            format!("{inp}{}", " ".repeat(chunk_size - (inp.len() % chunk_size)))
        };
    
        // The vector of vectors of the char codes of the padded input string.
        (0..inp_padded.len())
            .step_by(chunk_size)
            .map(|i| {
                inp_padded[i..i + chunk_size]
                    .chars()
                    .map(|c| c as u32 as f32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    } else {
        let mut inp_file = File::open(Path::new("10s.wav")).unwrap();
        let (header, data) = wav::read(&mut inp_file).unwrap();
        optional_header = Some(header);
        
        let BitDepth::Sixteen(inp) = data else {panic!()};
        inp_len = inp.len();
        
        let inp_padded = if inp.len() % chunk_size == 0 {
            inp
        } else {
            [inp, vec![0; chunk_size - (inp_len % chunk_size)]].concat()
        };

        (0..inp_padded.len())
            .step_by(chunk_size)
            .map(|i| {
                inp_padded[i..i + chunk_size]
                    .iter()
                    .map(|&c| c as f32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
        // let mut out_file = File::create(Path::new("data/output.wav")).unwrap();
        // wav::write(header, &data, &mut out_file).unwrap();
    }).transpose();

    // The input matrix.
    // inp_mat

    println!("{}", line!());
    // The encoded input matrix.
    let mut encoded = k.dot(&d);
    encoded.round(1);
    println!("{}", line!());
    
    let l = lu.tril();
    let u = lu.triu();

    // The decoded encoded input matrix.
    let mut decoded = FloatMatrix::from(
        (0..encoded.m)
            .map(|i| {
                let d = FloatMatrix::from(vec![encoded.transpose()[i].clone()]).transpose();
                FloatMatrix::solve_system(&l, &u, &d).transpose()[0].clone()
            })
            .collect::<Vec<_>>(),
    )
    .transpose();
    decoded.round(0);
    println!("{}", line!());
    
    
    if false {
        // The decoded string.
        let out = decoded
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
        println!("{}", &out[..inp_len.min(100)]);
    } else {
        let out = decoded
            .transpose()
            .data
            .iter()
            .flat_map(|row| {
                row.iter()
                    .map(|&c| c as i32 as i16)
                    .collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            
        let track = BitDepth::Sixteen(out);
            
        let mut out_file = File::create(Path::new("output.wav")).unwrap();
        wav::write(optional_header.unwrap(), &track, &mut out_file).unwrap();
    }
}
