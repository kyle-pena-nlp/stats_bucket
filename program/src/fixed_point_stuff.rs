use fixed::types::I32F32;
use fixed_macro::fixed;

pub const ZERO : I32F32 = I32F32::ZERO;
pub const ONE : I32F32  = I32F32::ONE;
pub const THREE: I32F32 = fixed!(3: I32F32);

pub fn into_fixed_point(x : i64) -> I32F32 {
    I32F32::from_bits(x)
}

pub fn into_i64(x : I32F32) -> i64 {
    x.to_bits()
}

pub fn vec_i64_to_vec_fixed_point(arr : &Vec<i64>) -> Vec<I32F32> {
    arr.iter().map(|x| into_fixed_point(*x)).collect()
}

pub fn into_fixed_point_array(arr : &[i64;10]) -> [I32F32;10] {
    arr.map(|x| into_fixed_point(x))
}

pub fn into_i64_array(arr : &[I32F32;10]) -> [i64;10] {
    arr.map(|x| into_i64(x))
}

pub trait FixedPowI {
    fn powi_positive(self, n : u32) -> Self;
}

impl FixedPowI for I32F32 {
    fn powi_positive(mut self, mut n : u32) -> Self {
        assert!(n > 0, "exponent should be positive");

        let mut acc = self;
        n -= 1;
    
        while n > 0 {
            if n & 1 == 1 {
                acc *= self;
            }
            self *= self;
            n >>= 1;
        }
        acc
    }
}