use std::ops::Add;

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{cast::ToPrimitive, identities::One, sign::Signed};
use rand::Rng;

const MAX_INT: u64 = i32::MAX as u64;

pub struct Optimus {
    prime: u64,
    mod_inverse: u64,
    random: u64,
}

impl Optimus {
    pub fn new(prime: u64, mod_inverse: Option<u64>, random: Option<u64>) -> Option<Self> {
        if prime >= MAX_INT || prime < 2 || (prime != 2 && prime & 1 != 1) {
            return None;
        }
        if let Some(r) = random {
            if r >= MAX_INT {
                return None;
            }
        }
        Some(Self {
            prime,
            mod_inverse: mod_inverse.or_else(|| math_mod_inverse(prime))?,
            random: random.unwrap_or_else(|| rand::thread_rng().gen_range(1 << 20..MAX_INT)),
        })
    }

    pub fn encode(&self, n: u64) -> u64 {
        ((n * self.prime) & MAX_INT) ^ self.random
    }

    pub fn decode(&self, n: u64) -> u64 {
        ((n ^ self.random) * self.mod_inverse) & MAX_INT
    }

    pub fn prime(&self) -> u64 {
        self.prime
    }

    pub fn mod_inverse(&self) -> u64 {
        self.mod_inverse
    }

    pub fn random(&self) -> u64 {
        self.random
    }
}

fn math_mod_inverse(prime: u64) -> Option<u64> {
    let m = BigInt::from(MAX_INT + 1);
    let egcm = BigInt::from(prime).extended_gcd(&m);
    if !egcm.gcd.is_one() {
        return None;
    }
    if egcm.x.is_negative() {
        return egcm.x.add(&m).to_u64();
    }
    egcm.x.to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endcode_decode() {
        let mut rng = rand::thread_rng();
        for v in &[
            (1580030173, Some(59260789), Some(1163945558)),
            (2123809381, Some(1885413229), Some(146808189)),
            (309779747, Some(49560203), Some(57733611)),
            (684934207, Some(1505143743), Some(846034763)),
            (743534599, Some(1356791223), Some(1336232185)),
            (54661037, Some(1342843941), Some(576322863)),
            (198194831, Some(229517423), Some(459462336)),

            (1580030173, None, None),
            (2123809381, None, None),
            (309779747, None, None),
            (684934207, None, None),
            (743534599, None, None),
            (54661037, None, None),
            (198194831, None, None),
        ] {
            let enc = Optimus::new(v.0, v.1, v.2).unwrap();
            let check = |n| {
                let new_id = enc.encode(n);
                assert_eq!(enc.decode(new_id), n);
            };
            for n in 0..10 {
                check(n);
            }
            for _ in 0..1000 {
                let n = rng.gen_range(10..MAX_INT - 10);
                check(n);
            }
            for n in MAX_INT - 10..MAX_INT {
                check(n);
            }
        }
    }

    #[test]
    fn test_math_mod_inverse() {
        assert_eq!(math_mod_inverse(1580030173).unwrap(), 59260789);
        assert_eq!(math_mod_inverse(2123809381).unwrap(), 1885413229);
        assert_eq!(math_mod_inverse(309779747).unwrap(), 49560203);
        assert_eq!(math_mod_inverse(684934207).unwrap(), 1505143743);
        assert_eq!(math_mod_inverse(743534599).unwrap(), 1356791223);
        assert_eq!(math_mod_inverse(54661037).unwrap(), 1342843941);
        assert_eq!(math_mod_inverse(198194831).unwrap(), 229517423);
    }
}
