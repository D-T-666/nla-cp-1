use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Mul, Div, Sub}, iter::Sum,
};

use pad::{Alignment, PadStr};
use rayon::prelude::*;
use crate::strat::{Field, RandomRange};

pub trait Exponent {
    fn dpow(self, e: i32) -> Self;
}

pub trait Round {
    fn dround(&self, decimals: usize) -> Self;
}

pub trait MatrixElement
where
    Self: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Sum
        + Field
        + RandomRange
        + Clone
        + Copy
        + Display
        + Exponent
        + Round
        + Send
        + Sync,
{
}

#[derive(Clone, PartialEq)]
pub struct Matrix<T>
where
    T: MatrixElement,
{
    pub n: usize,
    pub m: usize,
    pub data: Vec<Vec<T>>,
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: MatrixElement,
{
    pub fn from(data: Vec<Vec<T>>) -> Self {
        Self {
            n: data.len(),
            m: data[0].len(),
            data,
        }
    }

    pub fn zero(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            data: (0..n)
                .map(|_| (0..m).map(|_| T::ZERO).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    pub fn ones(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            data: (0..n)
                .map(|_| (0..m).map(|_| T::ONE).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    pub fn identity(n: usize) -> Self {
        Self {
            n,
            m: n,
            data: (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| T::from(if i == j { T::ONE } else { T::ZERO }))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn random(n: usize, m: usize, min: &T, max: &T) -> Self {
        let mut res = Self::zero(n, m);

        for i in 0..n {
            for j in 0..m {
                res.data[i][j] = T::rand_range(min, max);
            }
        }

        res
    }

    pub fn dot(&self, rhs: &Self) -> Self {
        // let rhs = rhs.transpose();

        // Matrix::from((0..self.n).into_par_iter().map(|i| {
        //     (0..rhs.n).map(|j| {
        //         self.data[i]
        //             .iter()
        //             .zip(&rhs.data[j])
        //             .map(|(&a, &b)| a * b)
        //             .sum()
        //     }).collect::<Vec<_>>()
        // }).collect::<Vec<_>>())

        Matrix::from((0..self.n).into_par_iter().map(|i| {
            (0..rhs.m).map(|j| {
                (0..self.m).map(|k| {
                    self.data[i][k] * rhs.data[k][j]
                }).sum()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>())
    }

    pub fn scale(mut self, amt: T) -> Self {
        for i in 0..self.n {
            for j in 0..self.m {
                self[i][j] = self[i][j] * amt;
            }
        }
        self
    }

    pub fn round(mut self, decimals: usize) -> Self {
        for i in 0..self.n {
            for j in 0..self.m {
                self[i][j] = self[i][j].dround(decimals);
            }
        }
        self
    }

    pub fn hadamard(self, rhs: Self) -> Self {
        if self.m != rhs.m || self.n != rhs.n {
            panic!("incompatible dimensions")
        }

        let mut res = Matrix::zero(self.n, self.m);

        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] * rhs[i][j];
            }
        }

        res
    }

    pub fn tril(&self) -> Self {
        let mut res = Self::zero(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                if j <= i {
                    res.data[i][j] = self[i][j];
                } else {
                    res.data[i][j] = T::ZERO;
                }
            }
        }
        res
    }

    pub fn triu(&self) -> Self {
        let mut res = Self::zero(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                if j >= i {
                    res.data[i][j] = self[i][j];
                } else {
                    res.data[i][j] = T::ZERO;
                }
            }
        }
        res
    }

    pub fn transpose(&self) -> Self {
        let mut res = Matrix::zero(self.m, self.n);
        for i in 0..self.n {
            for j in 0..self.m {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }
}

impl<T> Add for Matrix<T>
where
    T: MatrixElement,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.m != rhs.m || self.n != rhs.n {
            panic!("incompatible dimensions")
        }

        let mut res = Matrix::zero(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] + rhs[i][j];
            }
        }
        res
    }
}

impl<T> Sub for Matrix<T>
where
    T: MatrixElement,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.m != rhs.m || self.n != rhs.n {
            panic!("incompatible dimensions")
        }

        let mut res = Matrix::zero(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] - rhs[i][j];
            }
        }
        res
    }
}

impl<T> Index<usize> for Matrix<T>
where
    T: MatrixElement,
{
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T>
where
    T: MatrixElement,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Display for Matrix<T>
where
    T: MatrixElement,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elts = self
            .data
            .iter()
            .map(|row| row.iter().map(|elt| format!("{}", elt)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut max_len = elts[0][0].len();
        for i in 0..self.n {
            for j in 0..self.m {
                max_len = max_len.max(elts[i][j].len());
            }
        }
        write!(
            f,
            "{}",
            elts.iter()
                .map(|row| row
                    .iter()
                    .map(|elt| elt
                        .as_str()
                        .pad_to_width_with_alignment(max_len, Alignment::Right))
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl<T> Matrix<T>
where
    T: MatrixElement,
{
    fn back_substitution_u(mat: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
        let mut x = Matrix::zero(mat.n, 1);

        for i in (0..mat.n).rev() {
            x[i][0] = (b[i][0]
                - (i + 1..mat.n)
                    .map(|j| x[j][0] * mat[i][j])
                    .sum()) / mat[i][i];
        }

        x
    }

    fn forward_substitution_l(mat: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
        let mut x = Matrix::zero(mat.n, 1);

        for i in 0..mat.n {
            x[i][0] = (b[i][0]
                - (0..i)
                    .map(|j| x[j][0] * mat[i][j])
                    .sum()) / mat[i][i];
        }

        x
    }

    pub fn solve_system(l: &Matrix<T>, u: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
        Matrix::back_substitution_u(u, &Matrix::forward_substitution_l(l, b))
    }
}


impl<T> Matrix<T>
where
    T: MatrixElement,
{
    pub fn solve_system_iterative(a: &Matrix<T>, b: &Matrix<T>, omega: T, iterations: usize) -> Matrix<T> {    
        let mut x = b.clone();
        
        for _ in 0..iterations {
            for i in 0..a.n {
                let sum = (0..a.n).filter(|&j| j != i).map(|j| a[i][j] * x[j][0]).sum();
                x[i][0] = omega / a[i][i] * (b[i][0] - sum) + (T::ONE - omega) * x[i][0];
            }
        }
        
        x
    }
}


// Float matrix

use rand::Rng;

impl RandomRange for f32 {
    fn rand_range(min: &Self, max: &Self) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_range(*min..=*max)
    }
}

impl Field for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Exponent for f32 {
    fn dpow(self, e: i32) -> Self {
        self.powi(e)
    }
}

impl Round for f32 {
    fn dround(&self, decimals: usize) -> Self {
        let d = 10f32.powi(decimals as i32);
        (self * d as f32).round() / d as f32
    }
}

impl MatrixElement for f32 {}

pub type FloatMatrix = Matrix<f32>;

