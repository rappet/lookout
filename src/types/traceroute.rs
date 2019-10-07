use std::net::IpAddr;
use std::str::FromStr;

use crate::error::{LookoutError, Result};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TracerouteResult {
    pub destination: HostPair,
    pub max_hops: usize,
    pub packet_size: usize,
//    pub hops: Vec<Hop>
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HostPair {
    hostname: String,
    ip: IpAddr,
    asn: ASn,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TracerouteHop {

}