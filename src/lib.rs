#![feature(assoc_char_funcs)]

use std::convert::TryFrom;
use crate::NumSign::{NEGATIVE, POSITIVE};

enum NumSign {
    POSITIVE,
    NEGATIVE
}

impl NumSign {
    pub fn to_string(&self) -> String {
        match self {
            NumSign::POSITIVE => String::from(""),
            NumSign::NEGATIVE => String::from("-"),
        }
    }
}

struct BigNum {
    sign: NumSign,
    nums: Vec<u8>,
}

impl BigNum {
    fn from_string(mut num: String) -> BigNum {
        let sign = if num.chars().nth(0).expect("Less than 1 number supplied") == '-' {
            num.remove(0);
            NEGATIVE
        } else {
            POSITIVE
        };

        let nums: Vec<u8> = num.chars().into_iter()
            .map(|x| x.to_digit(10).expect("Bignum string contained a non-integer char"))
            .map(|x| u8::try_from(x).expect("Bignum char was larger than u8 can hold")).collect();
        BigNum {
            nums,
            sign
        }
    }

    fn to_string(&self) -> String {
        let mut number: String = self.nums.clone().into_iter()
            .map(|x| u32::from(x))
            .map(|x| char::from_digit(x, 10).expect("Could not convert vec item to char integer"))
            .collect();

        number.insert_str(0, &self.sign.to_string());

        number
    }
}

#[cfg(test)]
mod tests {
    use crate::BigNum;

    #[test]
    fn positive_from_and_to_string_is_identity() {
        let large_string = String::from("9").repeat(20);
        assert_eq!(BigNum::from_string(large_string.clone()).to_string(), large_string);
    }

    #[test]
    fn negative_from_and_to_string_is_identity() {
        let mut large_string = String::from("-");
        large_string.push_str(String::from("9").repeat(20).as_str());
        assert_eq!(BigNum::from_string(large_string.clone()).to_string(), large_string);
    }
}
