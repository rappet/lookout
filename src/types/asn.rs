use std::str::FromStr;

use crate::error::{LookoutError, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ASn(pub u16);

impl FromStr for ASn {
    type Err = LookoutError;

    fn from_str(s: &str) -> Result<ASn> {
        if s.len() > 2 && (s.starts_with("AS") || s.starts_with("as")) {
            Ok(ASn(s[2..].parse()?))
        } else {
            Ok(ASn(s.parse()?))
        }
    }
}

impl From<u16> for ASn {
    fn from(asn: u16) -> ASn {
        ASn(asn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_asn() {
        assert_eq!("123".parse::<ASn>().unwrap(), ASn(123));
        assert_eq!("AS123".parse::<ASn>().unwrap(), ASn(123));
        assert_eq!("as123".parse::<ASn>().unwrap(), ASn(123));
        assert!("AS".parse::<ASn>().is_err());
        assert!("As123".parse::<ASn>().is_err());
        assert!("".parse::<ASn>().is_err());
    }
}