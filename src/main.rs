mod strat;

// use std::{fs::File, path::Path};
// use wav::{self, BitDepth};

use strat::{Rational, RationalMatrix};

fn back_substitution_u(mat: RationalMatrix, b: RationalMatrix) -> RationalMatrix {
    let mut x = RationalMatrix::zero(mat.n, 1);

    for i in (0..mat.n).rev() {
        x[i][0] = b[i][0]
            - (i + 1..mat.n)
                .map(|j| x[j][0] * mat[i][j])
                .fold(Rational::from_int(0), |a, x| a + x);
    }

    x.simplified()
}

fn forward_substitution_l(mat: RationalMatrix, b: RationalMatrix) -> RationalMatrix {
    let mut x = RationalMatrix::zero(mat.n, 1);

    for i in 0..mat.n {
        x[i][0] = b[i][0]
            - (0..i)
                .map(|j| x[j][0] * mat[i][j])
                .fold(Rational::from_int(0), |a, x| a + x);
    }

    x.simplified()
}

fn main() {
    let chunk_size = 13;

    let d = RationalMatrix::identity(chunk_size);

    let l = RationalMatrix::random(
        chunk_size,
        chunk_size,
        Rational::from_ints(-1, chunk_size as i64),
        Rational::from_ints(1, chunk_size as i64),
    )
    .tril();
    let l = l.clone() - l.hadamard(d.clone()) + d.clone();

    let u = RationalMatrix::random(
        chunk_size,
        chunk_size,
        Rational::from_ints(-1, chunk_size as i64),
        Rational::from_ints(1, chunk_size as i64),
    )
    .triu();
    let u = u.clone() - u.hadamard(d.clone()) + d.clone();

    let k = l.clone().dot(u.clone()).unwrap();

    let input = "Hello, world!";
    let data = input
        .chars()
        .map(|c| Rational::from_int(c as i64))
        .collect::<Vec<_>>();
    #[allow(non_snake_case)]
    let D = RationalMatrix::from(vec![data]).transpose();

    let encoded = k.clone().dot(D.clone()).unwrap();
    
    
    let decoded = back_substitution_u(
        u.clone(),
        forward_substitution_l(l.clone(), encoded.clone()),
    );
    let decoded = decoded.transpose()[0]
        .iter()
        .map(|x| x.round().to_int() as u8 as char)
        .collect::<String>();

    println!("raw: {}", "Hello, world!");
    println!("encoded and decoded: {}", decoded);

    // // println!("{}", Rational::from_ints(3, 3).simplified() + Rational::from_int(7));
    // // println!("{}", RationalMatrix::one(4));
    // println!("{}", RationalMatrix::random(4, 4, Rational::from_int(-2), Rational::from_int(0)));

    // println!("{}", Rational::from_int(-2) - Rational::from_int(0));

    // let mut inp_file = File::open(Path::new("sound.wav")).unwrap();
    // let (header, data) = wav::read(&mut inp_file).unwrap();

    // println!("{:?}", header);
    // let BitDepth::Sixteen(data) = data else {
    //     panic!()
    // };
    // println!("{:?}", &data[..10]);
}
