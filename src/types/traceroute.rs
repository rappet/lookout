use std::net::IpAddr;
use std::str::FromStr;

use crate::error::{LookoutError, Result};
use crate::types::ASn;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TracerouteResult {
    pub header: TracerouteHeader,
    pub hops: Vec<TracerouteHop>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TracerouteHeader {
    pub destination: Host,
    pub max_hops: usize,
    pub packet_size: usize,
}

impl FromStr for TracerouteHeader {
    type Err = LookoutError;

    fn from_str(s: &str) -> Result<TracerouteHeader> {
        let mut components = s.split(',');
        let destination = components.next()
            .ok_or("traceroute output: destination misssing")?
            .trim_start_matches("traceroute to ")
            .trim()
            .parse()?;
        let max_hops = components.next()
            .ok_or("traceroute output: max hops misssing")?
            .trim().split(' ')
            .next().ok_or("traceroute output: max hops missing")?
            .parse()?;
        let packet_size = components.next()
            .ok_or("traceroute output: packet size")?
            .trim().split(' ')
            .next().ok_or("traceroute output: max hops missing")?
            .parse()?;
        Ok(TracerouteHeader {
            destination, max_hops, packet_size,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TracerouteHop {
    pub rtts: [Option<isize>; 3],
    pub hosts: [Option<Host>; 3],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub ip: IpAddr,
    pub asn: Option<ASn>,
}

impl Host {
    fn new (hostname: &str, ip: IpAddr, asn: Option<ASn>) -> Host {
        Host {
            hostname: String::from(hostname), ip, asn: asn,
        }
    }
}

impl FromStr for Host {
    type Err = LookoutError;

    fn from_str(s: &str) -> Result<Host> {
        let mut split = s.trim().split_whitespace();
        let hostname: &str = split.next().ok_or("traceroute output: missing hostname")?;
        let ip: IpAddr = {
            let ip = split.next().ok_or("traceroute output: missing ip address")?;
            if ip.len()==0 || ip.chars().next() != Some('(') || ip.chars().last() != Some(')') {
                Err("traceroute output: need parentheses aroung ip address")?;
            }
            ip[1..ip.len()-1].parse()?
        };
        let asn: Option<ASn> = if let Some(asn) = split.next() {
            if asn == "[*]" {
                // ASN could not be resolved
                None
            } else if asn.len()==0 || asn.chars().next() != Some('[') || asn.chars().last() != Some(']') {
                // Wrong format
                Err("traceroute output: need brackets aroung asn")?;
                unreachable!()
            } else {
                // Format is valid
                Some(asn[1..asn.len()-1].parse()?)
            }
        } else {
            None
        };
        if split.next().is_some() {
            Err("traceroute output: host has to many fields")?;
        }
        Ok(Host::new(hostname, ip, asn))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        assert_eq!(
            "traceroute to one.one.one.one (1.1.1.1), 30 hops max, 60 byte packets".parse::<TracerouteHeader>().unwrap(),
            TracerouteHeader {
                destination: Host::new("one.one.one.one", "1.1.1.1".parse().unwrap(), None),
                max_hops: 30,
                packet_size: 60,
            }
        );
    }

    #[test]
    fn test_parse_host() {
        assert_eq!(
            "one.one.one.one (1.1.1.1) [AS13335]".parse::<Host>().unwrap(),
            Host::new("one.one.one.one", "1.1.1.1".parse().unwrap(), Some(ASn(13335)))
        );
        assert_eq!(
            " 100.127.1.7 (100.127.1.7) [*]\t".parse::<Host>().unwrap(),
            Host::new("100.127.1.7", "100.127.1.7".parse().unwrap(), None)
        );
        assert_eq!(
            "100.127.1.7 (100.127.1.7)".parse::<Host>().unwrap(),
            Host::new("100.127.1.7", "100.127.1.7".parse().unwrap(), None)
        );
        assert!("100.127.1.7 100.127.1.7".parse::<Host>().is_err());
        assert!("100.127.1.7 (100.127.1.7) AS1234".parse::<Host>().is_err());
        assert!("100.127.1.7 (100.127.1.7) [AS1234] Blah".parse::<Host>().is_err());
    }
}