use flo_canvas::*;
use rand::*;
use std::env;
use std::ops::{Add, Mul, Sub};

const MAX_NUMBER_OF_PARTICLES: usize = 52;

pub fn read_args() -> usize {
    //! Reads the command line args, looking specifically
    //! for a passed in integer number of particles for the simulation
    let particle_count = match env::args().nth(1) {
        Some(number) => number.parse().unwrap_or(20),
        None => 20,
    };

    //For safety, we will cap the user at a max number of particles
    particle_count.min(MAX_NUMBER_OF_PARTICLES)
}

pub fn get_random_color() -> Color {
    //!Utility to get a Random Color for rendering purposes
    Color::Rgba(random::<f32>(), random::<f32>(), random::<f32>(), 1.0)
}

/// Quickly building a MathVec type for much more
///readable vectorized operations.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MathVec(pub f64, pub f64);

impl MathVec {
    pub fn inner_product(&self, other: &Self) -> f64 {
        // We are in the Real domain only, so we can ignore
        // the inner_product requirements for complex vectors.
        // This isnt't Quantum Mechanics!
        self.0 * other.0 + self.1 * other.1
    }

    pub fn distance(&self, other: &Self) -> f64 {
        //! Returns the distance between two MathVecs
        f64::sqrt((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2))
    }
}

impl Add for MathVec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for MathVec {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<MathVec> for f64 {
    type Output = MathVec;

    fn mul(self, rhs: MathVec) -> MathVec {
        MathVec(self * rhs.0, self * rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_color() {
        let rand_color = get_random_color();
        println!("{:?}", rand_color);
    }

    #[test]
    fn test_add_subtract_math_vec() {
        let v1 = MathVec(5.0, 3.2);
        let v2 = MathVec(4.7, -2.6);
        //Unfortunately, float equality is rather fickle to test.
        assert!((v1 + v2) - MathVec(9.7, 0.6) < MathVec(0.00001, 0.00001));
        assert!((v1 - v2) - MathVec(0.3, 5.8) < MathVec(0.00001, 0.00001));
    }

    #[test]
    fn test_inner_product() {
        let v1 = MathVec(5.0, 4.0);
        let v2 = MathVec(3.0, -7.0);
        assert_eq!(v1.inner_product(&v2), -13.0);
    }

    #[test]
    fn test_scalar_multiply() {
        let v1 = MathVec(5.0, 4.0);
        assert_eq!(3.0 * v1, MathVec(15.0, 12.0));
    }
}
