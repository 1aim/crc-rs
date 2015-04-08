#![cfg_attr(test, feature(test))]

//! # crc
//! Rust implementation of CRC(32, 64)
//!
//! ## Usage
//! ### Compute CRC32
//! ```rust
//! use crc::crc32;
//!
//! // CRC-32-IEEE is the most commonly used one
//! println!("{:x}", crc32::checksum_ieee(b"123456789")); // -> 0xcbf43926
//! println!("{:x}", crc32::checksum_castagnoli(b"123456789")); // -> 0xe3069283
//! println!("{:x}", crc32::checksum_koopman(b"123456789")); // -> 0x2d3dd0ae
//!
//! // use provided or custom polynomial
//! let mut digest = crc32::Digest::new(crc32::IEEE);
//! digest.write(b"123456789");
//! println!("{:x}", digest.sum32()); // -> 0xcbf43926
//! ```
//!
//! ### Compute CRC64
//! ```rust
//! use crc::crc64;
//!
//! println!("{:x}", crc64::checksum_ecma(b"123456789")); // -> 0x995dc9bbdf1939fa
//! println!("{:x}", crc64::checksum_iso(b"123456789")); // -> 0xb90956c775a41001
//!
//! // use provided or custom polynomial
//! let mut digest = crc64::Digest::new(crc64::ECMA);
//! digest.write(b"123456789");
//! println!("{:x}", digest.sum64()); // -> 0x995dc9bbdf1939fa
//! ```

#[cfg(test)] extern crate test;
#[macro_use]
extern crate lazy_static;

pub mod crc32;
pub mod crc64;

pub use self::crc32::Hasher32;
pub use self::crc64::Hasher64;
