use pad::{Alignment, PadStr};
use rand::Rng;
use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Mul, Sub}
};

#[derive(Clone, Copy)]
pub struct Rational {
    enumerator: u64,
    denominator: u64,
    negative: bool,
}

#[allow(dead_code)]
impl Rational {
    pub fn from_int(enumerator: i64) -> Self {
        Self {
            enumerator: enumerator.abs() as u64,
            denominator: 1,
            negative: enumerator.is_negative(),
        }
    }

    pub fn from_ints(enumerator: i64, denominator: i64) -> Self {
        Self {
            enumerator: enumerator.abs() as u64,
            denominator: denominator.abs() as u64,
            negative: enumerator.is_negative() ^ denominator.is_negative(),
        }
    }

    pub fn simplified(mut self) -> Self {
        let mut d = 2u64;
        while d * d <= self.enumerator.max(self.denominator) {
            if self.enumerator % d == 0 && self.denominator % d == 0 {
                self.enumerator /= d;
                self.denominator /= d;
            } else {
                d += 1;
            }
        }
        d = self.enumerator.min(self.denominator);
        if d > 0 && self.enumerator % d == 0 && self.denominator % d == 0 {
            self.enumerator /= d;
            self.denominator /= d;
        }
        if self.enumerator.max(self.denominator) > 1000000 {
            let mut d = 1000000;
            while d < self.enumerator.max(self.denominator) {
                d *= 10;
            }
            d /= 1000000;
            self.enumerator /= d;
            self.denominator /= d;
        }
        self
    }
    
    pub fn round(mut self) -> Self {
        self.enumerator = self.enumerator / self.denominator;
        self.denominator = 1;
        self
    }
    
    pub fn to_int(self) -> i64 {
        if self.negative {
            -(self.enumerator as i64 / self.denominator as i64)
        } else {
            self.enumerator as i64  / self.denominator as i64 
        }
    }
    
    pub fn rand_range(min: Self, max: Self) -> Self {
        let mut rng = rand::thread_rng();
        
        let denominator = rng.gen_range(3..11);
        let enumerator = rng.gen_range(0..denominator);
        
        (Rational::from_ints(enumerator, denominator) * (max - min)) + min
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        let a = self.simplified();
        let b = other.simplified();
        
        a.enumerator == b.enumerator &&
        a.denominator == b.denominator &&
        a.negative == b.negative
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.negative { "-" } else { "" },
            self.enumerator,
            if self.denominator != 1 { "/" } else { "" },
            if self.denominator != 1 {
                format!("{}", self.denominator)
            } else {
                format!("")
            }
        )
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        if self.negative == rhs.negative {
            Self {
                enumerator: self.enumerator * rhs.denominator + rhs.enumerator * self.denominator,
                denominator: self.denominator * rhs.denominator,
                negative: self.negative,
            }
        } else {
            let a = self.enumerator * rhs.denominator;
            let b = rhs.enumerator * self.denominator;
            let enumerator = if a > b { a - b } else { b - a };

            if enumerator != 0 {
                Self {
                    enumerator,
                    denominator: self.denominator * rhs.denominator,
                    negative: self.negative ^ (a < b),
                }
            } else {
                Self::from_int(0)
            }
        }.simplified()
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs * Rational::from_int(-1))
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.enumerator * rhs.enumerator != 0 {
            Self {
                enumerator: self.enumerator * rhs.enumerator,
                denominator: self.denominator * rhs.denominator,
                negative: self.negative ^ rhs.negative,
            }
        } else {
            Self::from_int(0)
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct RationalMatrix {
    pub n: usize,
    pub m: usize,
    pub data: Vec<Vec<Rational>>,
}

// impl RationalMatrix {
//     pub fn copy(&self) -> Self {
//         Self {
//             n: self.n,
//             m: self.m,
//             data: self.data.iter().map(|row| row.iter().map(|&x| x.clone()).collect::<Vec<_>>()).collect::<Vec<_>>()
//         }
//     }
// }

#[allow(dead_code)]
impl RationalMatrix {
    pub fn from(data: Vec<Vec<Rational>>) -> Self {
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
                .map(|_| (0..m).map(|_| Rational::from_int(0)).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    pub fn ones(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            data: (0..n)
                .map(|_| (0..m).map(|_| Rational::from_int(1)).collect::<Vec<_>>())
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
                        .map(|j| Rational::from_int(if i == j { 1 } else { 0 }))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn random(n: usize, m: usize, min: Rational, max: Rational) -> Self {
        let mut res = Self::zero(n, m);

        for i in 0..n {
            for j in 0..m {
                res[i][j] = Rational::rand_range(min, max);
            }
        }

        res
    }

    pub fn dot(self, rhs: Self) -> Option<Self> {
        if self.m != rhs.n {
            return None;
        }

        let mut res = RationalMatrix::zero(self.n, rhs.m);
        for i in 0..self.n {
            for j in 0..rhs.m {
                for k in 0..self.m {
                    res[i][j] = res[i][j] + self[i][k] * rhs[k][j];
                }
            }
        }
        
        Some(res.simplified())
    }
    
    pub fn scape(mut self, amt: Rational) -> Self {
        for i in 0..self.n {
            for j in 0..self.m {
                self[i][j] = self[i][j] * amt;
            }
        }
        self
    }

    pub fn hadamard(self, rhs: Self) -> Self {
        if self.m != rhs.m || self.n != rhs.n {
            panic!("incompatible dimensions")
        }

        let mut res = RationalMatrix::zero(self.n, self.m);

        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] * rhs[i][j];
            }
        }

        res
    }

    pub fn simplified(mut self) -> Self {
        for i in 0..self.n {
            for j in 0..self.m {
                self[i][j] = self[i][j].simplified();
            }
        }
        self
    }

    pub fn tril(mut self) -> Self {
        for i in 0..self.n {
            for j in 0..self.m {
                if j <= i {
                    self[i][j] = self[i][j].simplified();
                } else {
                    self[i][j] = Rational::from_int(0);
                }
            }
        }
        self
    }

    pub fn triu(mut self) -> Self {
        for i in 0..self.n {
            for j in 0..self.m {
                if j >= i {
                    self[i][j] = self[i][j].simplified();
                } else {
                    self[i][j] = Rational::from_int(0);
                }
            }
        }
        self
    }
    
    pub fn transpose(&self) -> Self {
        let mut res = RationalMatrix::zero(self.m, self.n);
        for i in 0..self.n {
            for j in 0..self.m {
                res[j][i] = self[i][j];
            }
        }
        res
    }
    
    pub fn round(mut self) -> Self {
        for i in 0..self.n {
            for j in 0..self.m {
                self[i][j] = self[i][j].round();
            }
        }
        self
    }
}

impl Add for RationalMatrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.m != rhs.m || self.n != rhs.n {
            panic!("incompatible dimensions")
        }

        let mut res = RationalMatrix::zero(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] + rhs[i][j];
            }
        }
        res
    }
}

impl Sub for RationalMatrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.m != rhs.m || self.n != rhs.n {
            panic!("incompatible dimensions")
        }

        let mut res = RationalMatrix::zero(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] - rhs[i][j];
            }
        }
        res
    }
}

impl Index<usize> for RationalMatrix {
    type Output = Vec<Rational>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for RationalMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Display for RationalMatrix {
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
