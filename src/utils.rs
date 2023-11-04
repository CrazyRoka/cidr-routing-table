use crate::errors::NetworkParseError;
use std::net::Ipv4Addr;

pub const MAX_LENGTH: u8 = 32;

pub fn get_cidr_mask(len: u8) -> Result<u32, NetworkParseError> {
    if len > MAX_LENGTH {
        Err(NetworkParseError::NetworkLengthError)
    } else {
        let right_len = MAX_LENGTH - len;
        let all_bits = u32::MAX as u64;
        let mask = (all_bits >> right_len) << right_len;

        Ok(mask as u32)
    }
}

pub fn cut_addr(addr: Ipv4Addr, len: u8) -> Result<Ipv4Addr, NetworkParseError> {
    if len > MAX_LENGTH {
        Err(NetworkParseError::NetworkLengthError)
    } else {
        let right_len = MAX_LENGTH - len;
        let bits = u32::from(addr);
        let new_bits = if right_len == MAX_LENGTH {
            0
        } else {
            (bits >> right_len) << right_len
        };

        Ok(Ipv4Addr::from(new_bits))
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::{cut_addr, get_cidr_mask, NetworkParseError};

    #[test]
    fn test_get_valid_cidr_mask() {
        let test_cases = [
            (0, 0b00000000000000000000000000000000),
            (1, 0b10000000000000000000000000000000),
            (4, 0b11110000000000000000000000000000),
            (5, 0b11111000000000000000000000000000),
            (10, 0b11111111110000000000000000000000),
            (28, 0b11111111111111111111111111110000),
            (31, 0b11111111111111111111111111111110),
            (32, 0b11111111111111111111111111111111),
        ];

        for (input, expected) in test_cases {
            let actual = get_cidr_mask(input);
            assert_eq!(
                Ok(expected),
                actual,
                "we expect cidr with math {input} to be {expected}"
            );
        }
    }

    #[test]
    fn test_get_invalid_cidr_mask() {
        let test_cases = [33, 34, 35, 50, 100];

        for input in test_cases {
            let actual = get_cidr_mask(input);
            assert_eq!(Err(NetworkParseError::NetworkLengthError), actual);
        }
    }

    #[test]
    fn test_cut_valid_addr() {
        let test_cases = [
            (
                0b11001101010111100001101010111010,
                7,
                0b11001100000000000000000000000000,
            ),
            (
                0b11001101010111100001101010111010,
                0,
                0b00000000000000000000000000000000,
            ),
            (
                0b11001101010111100001101010111010,
                32,
                0b11001101010111100001101010111010,
            ),
            (
                0b11001101010111100001101010111010,
                30,
                0b11001101010111100001101010111000,
            ),
        ];

        for (input, len, expected) in test_cases {
            let addr = Ipv4Addr::from(input);
            let expected_addr = Ipv4Addr::from(expected);
            let actual = cut_addr(addr, len);

            assert_eq!(actual, Ok(expected_addr));
        }
    }

    #[test]
    fn test_cut_invalid_addr() {
        let test_cases = [33, 35, 100, u8::MAX];

        for len in test_cases {
            let addr = Ipv4Addr::new(127, 0, 0, 1);
            let actual = cut_addr(addr, len);

            assert_eq!(actual, Err(NetworkParseError::NetworkLengthError));
        }
    }
}
