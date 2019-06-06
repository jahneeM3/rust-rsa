extern crate num_bigint;

use std::io::{self, Read};
use num_bigint::{BigUint};


fn main() -> io::Result<()> {
    // parsing gpg privatekey
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    let ns = &buffer[11..(11+128)];
    let es = &buffer[(11+128+2)..(11+128+2+3)];
    let n = BigUint::from_bytes_le(ns);
    let e = BigUint::from_bytes_le(es);

    println!("{} {}", n, e);

    // find private key tag
    //              first bit of tag always set
    //            /
    //           |              /-\ = length-type = 1, two-octet length
    // 0x95 = || 1 0 0 1 // 0 1 0 1 ||
    //               \tag = 5/
    //  next two bytes encode length, 0x01d8 = 472

    // Extract Public Key information

    // Extract Private Key information
    // Find Packet Tag 7

    // 0x97 = || 1 0 0 1 // 1 1 0 1 ||
    // bits 5-2 must encode 7
    Ok(())
}
