/* ---------------------------------------------------------------- *
 * CONSTANTS                                                        *
 * ---------------------------------------------------------------- */

#![allow(non_snake_case, non_upper_case_globals)]
use hacl_star::{chacha20poly1305, curve25519};

pub const DHLEN: usize = curve25519::SECRET_LENGTH;
pub(crate) const HASHLEN: usize = 32;
pub(crate) const BLOCKLEN: usize = 64;
pub(crate) const EMPTY_HASH: [u8; DHLEN] = [0_u8; HASHLEN];
pub(crate) const EMPTY_KEY: [u8; DHLEN] = [0_u8; DHLEN];
pub const MAC_LENGTH: usize = chacha20poly1305::MAC_LENGTH;
pub(crate) const MAX_MESSAGE: usize = 0xFFFF;
pub(crate) const MAX_NONCE: u64 = u64::max_value();
pub(crate) const NONCE_LENGTH: usize = chacha20poly1305::NONCE_LENGTH;
pub(crate) const ZEROLEN: [u8; 0] = [0_u8; 0];
pub(crate) const forbidden_curve_values: [[u8; 32]; 12] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    [
        224, 235, 122, 124, 59, 65, 184, 174, 22, 86, 227, 250, 241, 159, 196, 106, 218, 9, 141,
        235, 156, 50, 177, 253, 134, 98, 5, 22, 95, 73, 184, 0,
    ],
    [
        95, 156, 149, 188, 163, 80, 140, 36, 177, 208, 177, 85, 156, 131, 239, 91, 4, 68, 92, 196,
        88, 28, 142, 134, 216, 34, 78, 221, 208, 159, 17, 87,
    ],
    [
        236, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
    ],
    [
        237, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
    ],
    [
        238, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
    ],
    [
        205, 235, 122, 124, 59, 65, 184, 174, 22, 86, 227, 250, 241, 159, 196, 106, 218, 9, 141,
        235, 156, 50, 177, 253, 134, 98, 5, 22, 95, 73, 184, 128,
    ],
    [
        76, 156, 149, 188, 163, 80, 140, 36, 177, 208, 177, 85, 156, 131, 239, 91, 4, 68, 92, 196,
        88, 28, 142, 134, 216, 34, 78, 221, 208, 159, 17, 215,
    ],
    [
        217, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ],
    [
        218, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ],
    [
        219, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ],
];
