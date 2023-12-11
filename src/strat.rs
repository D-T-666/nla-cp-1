// use pad::{Alignment, PadStr};
// use rand::Rng;
// use std::{
//     fmt::Display,
//     ops::{Add, Index, IndexMut, Mul, Sub}
// };

pub trait RandomRange {
    /// Returns a random value of the implementer type in the range of `min` and `max`
    fn rand_range(min: &Self, max: &Self) -> Self;
}

pub trait Field {
    const ZERO: Self;
    const ONE: Self;
}

// #[derive(Clone, Copy)]
// pub struct Rational {
//     m: u16,
//     e: u16,
//     s: bool,
// }

// #[allow(dead_code)]
// impl Rational {
//     pub fn from_int(int: i32) -> Self {
//         Self {
//             m: (int as usize % (10usize.pow(int.ilog10() - 1))) as u16,
//             e: int.ilog10() as u16 - 1,
//             s: int.is_negative()
//         }
//     }

//     pub fn from_float(float: f32) -> Self {
//         Self {
//             m: (float % 10.0f32.powf(float.log10().round() - 1.0)) as u16,
//             e: float.log10().round() as u16 - 1,
//             s: float.is_sign_negative(),
//         }
//     }

//     pub fn rand_range(min: Self, max: Self) -> Self {
//         let mut rng = rand::thread_rng();

//         let denominator = rng.gen_range(min.e..max.e);
//         let enumerator = rng.gen_range(if );

//         (Rational::from_ints(enumerator, denominator) * (max - min)) + min
//     }
// }

// impl PartialEq for Rational {
//     fn eq(&self, other: &Self) -> bool {
//         let a = self.simplified();
//         let b = other.simplified();

//         a.enumerator == b.enumerator &&
//         a.denominator == b.denominator &&
//         a.negative == b.negative
//     }
// }

// impl Display for Rational {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}{}{}{}",
//             if self.negative { "-" } else { "" },
//             self.enumerator,
//             if self.denominator != 1 { "/" } else { "" },
//             if self.denominator != 1 {
//                 format!("{}", self.denominator)
//             } else {
//                 format!("")
//             }
//         )
//     }
// }

// impl Add for Rational {
//     type Output = Rational;

//     fn add(self, rhs: Self) -> Self::Output {
//         if self.negative == rhs.negative {
//             Self {
//                 enumerator: self.enumerator * rhs.denominator + rhs.enumerator * self.denominator,
//                 denominator: self.denominator * rhs.denominator,
//                 negative: self.negative,
//             }
//         } else {
//             let a = self.enumerator * rhs.denominator;
//             let b = rhs.enumerator * self.denominator;
//             let enumerator = if a > b { a - b } else { b - a };

//             if enumerator != 0 {
//                 Self {
//                     enumerator,
//                     denominator: self.denominator * rhs.denominator,
//                     negative: self.negative ^ (a < b),
//                 }
//             } else {
//                 Self::from_int(0)
//             }
//         }.simplified()
//     }
// }

// impl Sub for Rational {
//     type Output = Rational;

//     fn sub(self, rhs: Self) -> Self::Output {
//         self + (rhs * Rational::from_int(-1))
//     }
// }

// impl Mul for Rational {
//     type Output = Rational;

//     fn mul(self, rhs: Self) -> Self::Output {
//         if self.enumerator * rhs.enumerator != 0 {
//             Self {
//                 enumerator: self.enumerator * rhs.enumerator,
//                 denominator: self.denominator * rhs.denominator,
//                 negative: self.negative ^ rhs.negative,
//             }
//         } else {
//             Self::from_int(0)
//         }
//     }
// }
