mod matrix;
mod strat;

use matrix::{Exponent, Matrix, MatrixElement, Round};
use rand::Rng;
use strat::{Field, RandomRange};

impl RandomRange for f64 {
    fn rand_range(min: &Self, max: &Self) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_range(*min..=*max)
    }
}

impl Field for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Exponent for f64 {
    fn dpow(self, e: i32) -> Self {
        self.powi(e)
    }
}

impl Round for f64 {
    fn dround(&self, decimals: usize) -> Self {
        let d = 10f64.powi(decimals as i32);
        (self * d as f64).round() / d as f64
    }
}

impl MatrixElement for f64 {}

fn main() {
    let chunk_size = 1000;

    println!("{}", line!());
    let l = Matrix::random(chunk_size, chunk_size, &-1.0, &1.0)
        .tril()
        .scale(1.0 / chunk_size as f64);
    let mut l = l.clone() - l.hadamard(Matrix::identity(chunk_size)) + Matrix::identity(chunk_size);
    l.round(1);
    println!("{}", line!());

    let u = Matrix::random(chunk_size, chunk_size, &-1.0, &1.0)
        .triu()
        .scale(1.0 / chunk_size as f64);
    let mut u = u.clone() - u.hadamard(Matrix::identity(chunk_size)) + Matrix::identity(chunk_size);
    u.round(1);
    println!("{}", line!());

    let k = l.clone().dot(&u);
    println!("{}", line!());

    // println!("{}", l);
    // println!("{}", u);
    // println!("{}", k);
    // I am your father!
    // - NOOOOO

    // The input string.
    let inp = "Gamarjoba, chemo mayurebelo!";

    // The input string, but padded.
    let inp_padded = if inp.len() % chunk_size == 0 {
        String::from(inp)
    } else {
        format!("{inp}{}", " ".repeat(chunk_size - (inp.len() % chunk_size)))
    };

    // The vector of vectors of the char codes of the padded input string.
    let inp_padded = (0..inp_padded.len())
        .step_by(chunk_size)
        .map(|i| {
            inp_padded[i..i + chunk_size]
                .chars()
                .map(|c| c as i64 as f64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // The input matrix.
    let d = Matrix::from(inp_padded).transpose();

    println!("{}", line!());
    // The encoded input matrix.
    let mut encoded = k.dot(&d);
    encoded.round(1);
    println!("{}", line!());

    // The decoded encoded input matrix.
    let mut decoded = Matrix::from(
        (0..encoded.m)
            .map(|i| {
                let d = Matrix::from(vec![encoded.transpose()[i].clone()]).transpose();
                Matrix::solve_system(&l, &u, &d).transpose()[0].clone()
            })
            .collect::<Vec<_>>(),
    )
    .transpose();
    decoded.round(0);

    // The decoded string.
    let out = decoded
        .transpose()
        .data
        .iter()
        .map(|row| {
            row.iter()
                .map(|&c| c as i64 as u8 as char)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("");

    println!("{}", &out[..inp.len()]);
}
