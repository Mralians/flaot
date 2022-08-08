#![allow(dead_code)]
#![allow(unused_variables)]

pub mod float32 {
    pub struct Float32 {
        pub sign: f32,
        pub exponent: f32,
        pub mantissa: f32,
    }
    fn to_parts(f: f32) -> (u32, u32, u32) {
        let f_bits = f.to_bits();
        let sign = f_bits >> 31;
        let exponent = (f_bits >> 23) & 0xff;
        let mantissa = f_bits & 0x7FFFFF;

        (sign, exponent, mantissa)
    }
    impl std::fmt::Display for Float32 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "SIGN: {}, EXPONENT: {}, MANTISSA: {}",
                self.sign, self.exponent, self.mantissa
            )
        }
    }
    impl Float32 {
        const BIAS: f32 = 127.0;
        const RADIX: f32 = 2.0;
        pub fn new(sign: f32, exponent: f32, mantissa: f32) -> Self {
            Self {
                sign,
                exponent,
                mantissa,
            }
        }
        pub fn decode(f: f32) -> Self {
            let parts = to_parts(f);

            let sign = (-1.0_f32).powf(parts.0 as f32);
            let exponent = (Self::RADIX).powf(parts.1 as f32 - Self::BIAS);

            let mut mantissa: f32 = 1.0;
            for i in 0..23 {
                let mask = 1 << i;
                let bit_i = parts.2 & mask;
                if bit_i != 0 {
                    let weight = 2_f32.powf(i as f32 - 23.0);
                    mantissa += weight;
                }
            }

            Self {
                sign,
                exponent,
                mantissa,
            }
        }
        pub fn from_parts(&self) -> f32 {
            self.sign * self.exponent * self.mantissa
        }
    }
}
#[cfg(test)]
mod tests {
    use super::float32::Float32;
    #[test]
    fn test_from_parts() {
        let f = Float32::new(1.0, 32.0, 1.325625);
        assert_eq!(42.42, f.from_parts());
    }
    #[test]
    fn test_to_parts() {
        let decode = Float32::decode(42.42);
        let decode = (decode.sign, decode.exponent, decode.mantissa);
        assert_eq!((1.0, 32.0, 1.325625), decode);
    }
}
