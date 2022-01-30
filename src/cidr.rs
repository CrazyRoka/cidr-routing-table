use std::{net::Ipv4Addr, str::FromStr};

use crate::{
    errors::NetworkParseError,
    utils::{get_cidr_mask, MAX_LENGTH},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Ipv4Cidr {
    addr: Ipv4Addr,
    len: u8,
}

impl Ipv4Cidr {
    pub fn new(addr: Ipv4Addr, len: u8) -> Result<Self, NetworkParseError> {
        let mask = get_cidr_mask(len)?;
        let bits = u32::from(addr);

        if (bits & mask) != bits {
            Err(NetworkParseError::NetworkLengthError)
        } else {
            Ok(Self { addr, len })
        }
    }

    pub fn new_host(addr: Ipv4Addr) -> Self {
        Self {
            addr,
            len: MAX_LENGTH,
        }
    }

    pub fn prefix_len(&self) -> u8 {
        self.len
    }

    pub fn min(&self) -> Ipv4Addr {
        self.addr
    }

    pub fn max(&self) -> Ipv4Addr {
        let bits = u32::from(self.addr);
        let mask = get_cidr_mask(self.len)
            .unwrap_or_else(|_| panic!("{} should always be lower than or equal to 32", self.len));
        let reversed_mask = u32::MAX ^ mask;

        let max_bits = bits | reversed_mask;
        Ipv4Addr::from(max_bits)
    }

    pub fn contains(&self, addr: Ipv4Addr) -> bool {
        let lower = self.min();
        let upper = self.max();

        lower <= addr && addr <= upper
    }
}

impl FromStr for Ipv4Cidr {
    type Err = NetworkParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();

        if parts.len() != 2 {
            return Err(NetworkParseError::CidrParseError);
        }

        let addr = Ipv4Addr::from_str(parts[0]).map_err(NetworkParseError::AddrParseError)?;
        let len = parts[1]
            .parse::<u8>()
            .map_err(NetworkParseError::ParseIntError)?;

        Self::new(addr, len)
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::NetworkParseError;

    use super::{Ipv4Cidr, MAX_LENGTH};
    use std::{net::Ipv4Addr, str::FromStr};

    #[test]
    fn test_create_ipv4_cidr() {
        let test_cases = [
            (Ipv4Addr::new(0, 0, 0, 0), 0),
            (Ipv4Addr::new(0, 0, 0, 0), 8),
            (Ipv4Addr::new(0, 0, 0, 0), 32),
            (Ipv4Addr::new(192, 168, 0, 0), 16),
            (Ipv4Addr::new(192, 168, 0, 0), 13),
            (Ipv4Addr::new(192, 168, 200, 4), 30),
            (Ipv4Addr::new(192, 168, 200, 8), 30),
            (Ipv4Addr::new(169, 254, 0, 0), 16),
            (Ipv4Addr::new(127, 0, 0, 0), 8),
            (Ipv4Addr::new(100, 64, 0, 0), 10),
        ];

        for (addr, len) in test_cases {
            let cidr = Ipv4Cidr::new(addr, len);
            assert_eq!(
                cidr,
                Ok(Ipv4Cidr { addr, len }),
                "we expect {addr} with cidr mask len {len} to be valid"
            );
        }
    }

    #[test]
    fn test_create_invalid_ipv4_cidr() {
        let test_cases = [
            (Ipv4Addr::new(192, 168, 0, 0), 100),
            (Ipv4Addr::new(192, 168, 0, 0), 0),
            (Ipv4Addr::new(192, 168, 0, 0), 12),
            (Ipv4Addr::new(192, 168, 0, 0), 11),
            (Ipv4Addr::new(192, 168, 200, 4), 29),
            (Ipv4Addr::new(192, 168, 200, 8), 10),
            (Ipv4Addr::new(169, 254, 0, 0), 10),
            (Ipv4Addr::new(127, 0, 0, 0), 7),
            (Ipv4Addr::new(100, 64, 0, 0), 9),
        ];

        for (addr, len) in test_cases {
            let cidr = Ipv4Cidr::new(addr, len);
            assert_eq!(
                cidr,
                Err(NetworkParseError::NetworkLengthError),
                "we expect {addr} with cidr mask len {len} to be invalid"
            );
        }
    }

    #[test]
    fn test_create_host_cidr() {
        let test_cases = [
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 200, 4),
            Ipv4Addr::new(192, 168, 200, 8),
            Ipv4Addr::new(169, 254, 0, 0),
            Ipv4Addr::new(127, 0, 0, 0),
            Ipv4Addr::new(100, 64, 0, 0),
        ];

        for addr in test_cases {
            let cidr = Ipv4Cidr::new_host(addr);
            assert_eq!(
                cidr,
                Ipv4Cidr {
                    addr,
                    len: MAX_LENGTH
                },
                "we expect {addr} to be converted to cidr with length {MAX_LENGTH}"
            );
        }
    }

    #[test]
    fn test_len() {
        let test_cases = [
            (Ipv4Addr::new(0, 0, 0, 0), 0),
            (Ipv4Addr::new(0, 0, 0, 0), 8),
            (Ipv4Addr::new(0, 0, 0, 0), 32),
            (Ipv4Addr::new(192, 168, 0, 0), 16),
            (Ipv4Addr::new(192, 168, 0, 0), 13),
            (Ipv4Addr::new(192, 168, 200, 4), 30),
            (Ipv4Addr::new(192, 168, 200, 8), 30),
            (Ipv4Addr::new(169, 254, 0, 0), 16),
            (Ipv4Addr::new(127, 0, 0, 0), 8),
            (Ipv4Addr::new(100, 64, 0, 0), 10),
        ];

        for (addr, len) in test_cases {
            let cidr = Ipv4Cidr::new(addr, len).unwrap();
            let actual = cidr.prefix_len();
            assert_eq!(actual, len, "we expect {actual} to equal {len}");
        }
    }

    #[test]
    fn test_min() {
        let test_cases = [
            (Ipv4Addr::new(0, 0, 0, 0), 0),
            (Ipv4Addr::new(0, 0, 0, 0), 8),
            (Ipv4Addr::new(0, 0, 0, 0), 32),
            (Ipv4Addr::new(192, 168, 0, 0), 16),
            (Ipv4Addr::new(192, 168, 0, 0), 13),
            (Ipv4Addr::new(192, 168, 200, 4), 30),
            (Ipv4Addr::new(192, 168, 200, 8), 30),
            (Ipv4Addr::new(169, 254, 0, 0), 16),
            (Ipv4Addr::new(127, 0, 0, 0), 8),
            (Ipv4Addr::new(100, 64, 0, 0), 10),
        ];

        for (addr, len) in test_cases {
            let cidr = Ipv4Cidr::new(addr, len).unwrap();
            let actual = cidr.min();
            assert_eq!(actual, addr, "we expect {actual} to equal {addr}");
        }
    }

    #[test]
    fn test_max() {
        let test_cases = [
            (
                Ipv4Addr::new(0, 0, 0, 0),
                0,
                Ipv4Addr::new(255, 255, 255, 255),
            ),
            (
                Ipv4Addr::new(0, 0, 0, 0),
                8,
                Ipv4Addr::new(0, 255, 255, 255),
            ),
            (Ipv4Addr::new(0, 0, 0, 0), 32, Ipv4Addr::new(0, 0, 0, 0)),
            (
                Ipv4Addr::new(192, 168, 0, 0),
                16,
                Ipv4Addr::new(192, 168, 255, 255),
            ),
            (
                Ipv4Addr::new(192, 168, 0, 0),
                13,
                Ipv4Addr::new(192, 175, 255, 255),
            ),
            (
                Ipv4Addr::new(192, 168, 200, 4),
                30,
                Ipv4Addr::new(192, 168, 200, 7),
            ),
            (
                Ipv4Addr::new(192, 168, 200, 8),
                30,
                Ipv4Addr::new(192, 168, 200, 11),
            ),
            (
                Ipv4Addr::new(169, 254, 0, 0),
                16,
                Ipv4Addr::new(169, 254, 255, 255),
            ),
            (
                Ipv4Addr::new(127, 0, 0, 0),
                8,
                Ipv4Addr::new(127, 255, 255, 255),
            ),
            (
                Ipv4Addr::new(100, 64, 0, 0),
                10,
                Ipv4Addr::new(100, 127, 255, 255),
            ),
        ];

        for (addr, len, expected) in test_cases {
            let cidr = Ipv4Cidr::new(addr, len).unwrap();
            let actual = cidr.max();
            assert_eq!(actual, expected, "we expect {actual} to equal {expected}");
        }
    }

    #[test]
    fn test_parse_str() {
        let test_cases = [
            ("192.168.0.0/16", Ipv4Addr::new(192, 168, 0, 0), 16),
            ("192.168.0.0/13", Ipv4Addr::new(192, 168, 0, 0), 13),
            ("192.168.200.4/30", Ipv4Addr::new(192, 168, 200, 4), 30),
            ("192.168.200.8/30", Ipv4Addr::new(192, 168, 200, 8), 30),
            ("169.254.0.0/16", Ipv4Addr::new(169, 254, 0, 0), 16),
            ("127.0.0.0/8", Ipv4Addr::new(127, 0, 0, 0), 8),
            ("100.64.0.0/10", Ipv4Addr::new(100, 64, 0, 0), 10),
        ];

        for (cidr_str, addr, len) in test_cases {
            let cidr = Ipv4Cidr::from_str(cidr_str);
            let expected = Ipv4Cidr { addr, len };

            assert_eq!(
                cidr,
                Ok(expected),
                "we expect {cidr_str} to be correctly parsed"
            );
        }
    }

    #[test]
    fn test_parse_invalid_str() {
        let test_cases = [
            (
                "192.168.0.0/100",
                Err(NetworkParseError::NetworkLengthError),
            ),
            ("192.168.0.0/0", Err(NetworkParseError::NetworkLengthError)),
            ("192.168.0.0/12", Err(NetworkParseError::NetworkLengthError)),
            ("192.168.0.0/11", Err(NetworkParseError::NetworkLengthError)),
            (
                "192.168.200.4/29",
                Err(NetworkParseError::NetworkLengthError),
            ),
            (
                "192.168.200.8/10",
                Err(NetworkParseError::NetworkLengthError),
            ),
            ("169.254.0.0/10", Err(NetworkParseError::NetworkLengthError)),
            ("127.0.0.0/7", Err(NetworkParseError::NetworkLengthError)),
            ("100.64.0.0/9", Err(NetworkParseError::NetworkLengthError)),
            (
                "invalid/12",
                Err(NetworkParseError::AddrParseError(
                    "invalid".parse::<Ipv4Addr>().err().unwrap(),
                )),
            ),
            (
                "128.256.3.4/12",
                Err(NetworkParseError::AddrParseError(
                    "128.256.3.4".parse::<Ipv4Addr>().err().unwrap(),
                )),
            ),
            ("wrong", Err(NetworkParseError::CidrParseError)),
            (
                "169.254.0.0/hello",
                Err(NetworkParseError::ParseIntError(
                    "hello".parse::<u8>().err().unwrap(),
                )),
            ),
        ];

        for (cidr_str, expected) in test_cases {
            let cidr = Ipv4Cidr::from_str(cidr_str);
            assert_eq!(cidr, expected, "we expect {cidr_str} to be invalid");
        }
    }

    #[test]
    fn test_contains_addr() {
        let test_cases = [
            ("192.168.0.0/16", Ipv4Addr::new(192, 168, 0, 0)),
            ("192.168.0.0/16", Ipv4Addr::new(192, 168, 0, 123)),
            ("192.168.0.0/16", Ipv4Addr::new(192, 168, 0, 255)),
            ("192.168.0.0/16", Ipv4Addr::new(192, 168, 255, 0)),
            ("192.168.0.0/16", Ipv4Addr::new(192, 168, 255, 123)),
            ("192.168.0.0/16", Ipv4Addr::new(192, 168, 255, 255)),
            ("192.168.0.0/13", Ipv4Addr::new(192, 169, 0, 0)),
            ("192.168.200.4/30", Ipv4Addr::new(192, 168, 200, 4)),
            ("192.168.200.4/30", Ipv4Addr::new(192, 168, 200, 5)),
            ("192.168.200.4/30", Ipv4Addr::new(192, 168, 200, 6)),
            ("192.168.200.4/30", Ipv4Addr::new(192, 168, 200, 7)),
            ("192.168.200.8/30", Ipv4Addr::new(192, 168, 200, 8)),
            ("192.168.200.8/30", Ipv4Addr::new(192, 168, 200, 9)),
            ("192.168.200.8/30", Ipv4Addr::new(192, 168, 200, 10)),
            ("192.168.200.8/30", Ipv4Addr::new(192, 168, 200, 11)),
            ("169.254.0.0/16", Ipv4Addr::new(169, 254, 0, 0)),
            ("127.0.0.0/8", Ipv4Addr::new(127, 0, 0, 0)),
            ("100.64.0.0/10", Ipv4Addr::new(100, 64, 0, 0)),
            ("0.0.0.0/0", Ipv4Addr::new(0, 0, 0, 0)),
            ("0.0.0.0/0", Ipv4Addr::new(255, 255, 255, 255)),
        ];

        for (cidr_str, addr) in test_cases {
            let cidr = Ipv4Cidr::from_str(cidr_str).expect("Cidr is correct");
            let result = cidr.contains(addr);

            assert!(result, "we expect {cidr:?} to contain {addr}");
        }
    }
}
