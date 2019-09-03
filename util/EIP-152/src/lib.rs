// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

/// the precomputed values for BLAKE2b
/// there are 10 16-byte arrays - one for each round
/// the entries are calculated from the sigma constants.
const SIGMA: [[usize; 16]; 10] = [
	[0, 2, 4, 6, 1, 3, 5, 7, 8, 10, 12, 14, 9, 11, 13, 15],
	[14, 4, 9, 13, 10, 8, 15, 6, 1, 0, 11, 5, 12, 2, 7, 3],
	[11, 12, 5, 15, 8, 0, 2, 13, 10, 3, 7, 9, 14, 6, 1, 4],
	[7, 3, 13, 11, 9, 1, 12, 14, 2, 5, 4, 15, 6, 10, 0, 8],
	[9, 5, 2, 10, 0, 7, 4, 15, 14, 11, 6, 3, 1, 12, 8, 13],
	[2, 6, 0, 8, 12, 10, 11, 3, 4, 7, 15, 1, 13, 5, 14, 9],
	[12, 1, 14, 4, 5, 15, 13, 10, 0, 6, 9, 8, 7, 3, 2, 11],
	[13, 7, 12, 3, 11, 14, 1, 9, 5, 15, 8, 2, 0, 4, 6, 10],
	[6, 14, 11, 0, 15, 9, 3, 8, 12, 13, 1, 10, 2, 7, 4, 5],
	[10, 8, 7, 1, 2, 4, 6, 5, 15, 9, 3, 13, 11, 14, 12, 0],
];


/// IV is an initialization vector for BLAKE2b
const IV: [u64; 8] = [
	0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
	0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
];

/// F is a compression function for BLAKE2b. It takes as an argument the state
/// vector `h`, message block vector `m`, offset counter `t`, final
/// block indicator flag `f`, and number of rounds `rounds`. The state vector
/// provided as the first parameter is modified by the function.
pub fn compress(h: &mut [u64; 8], m: [u64; 16], t: [u64; 2], f: bool, rounds: usize) {
	let mut v = Vec::new();
	v.extend(h.iter().cloned());
	v.extend_from_slice(&IV);

	v[12] ^= t[0];
	v[13] ^= t[1];

	if f {
		v[14] = !v[14]
	}

	for i in 0..rounds {
		let s = &SIGMA[i % 10];
		v[0] = v[0].wrapping_add(m[s[0]]);
		v[0] = v[0].wrapping_add(v[4]);
		v[12] ^= v[0];
		v[12] = v[12].rotate_right(32);
		v[8] = v[8].wrapping_add(v[12]);
		v[4] ^= v[8];
		v[4] = v[4].rotate_right(24);
		v[1] = v[1].wrapping_add(m[s[1]]);
		v[1] = v[1].wrapping_add(v[5]);
		v[13] ^= v[1];
		v[13] = v[13].rotate_right(32);
		v[9] = v[9].wrapping_add(v[13]);
		v[5] ^= v[9];
		v[5] = v[5].rotate_right(24);
		v[2] = v[2].wrapping_add(m[s[2]]);
		v[2] = v[2].wrapping_add(v[6]);
		v[14] ^= v[2];
		v[14] = v[14].rotate_right(32);
		v[10] = v[10].wrapping_add(v[14]);
		v[6] ^= v[10];
		v[6] = v[6].rotate_right(24);
		v[3] = v[3].wrapping_add(m[s[3]]);
		v[3] = v[3].wrapping_add(v[7]);
		v[15] ^= v[3];
		v[15] = v[15].rotate_right(32);
		v[11] = v[11].wrapping_add(v[15]);
		v[7] ^= v[11];
		v[7] = v[7].rotate_right(24);
		v[0] = v[0].wrapping_add(m[s[4]]);
		v[0] = v[0].wrapping_add(v[4]);
		v[12] ^= v[0];
		v[12] = v[12].rotate_right(16);
		v[8] = v[8].wrapping_add(v[12]);
		v[4] ^= v[8];
		v[4] = v[4].rotate_right(63);
		v[1] = v[1].wrapping_add(m[s[5]]);
		v[1] = v[1].wrapping_add(v[5]);
		v[13] ^= v[1];
		v[13] = v[13].rotate_right(16);
		v[9] = v[9].wrapping_add(v[13]);
		v[5] ^= v[9];
		v[5] = v[5].rotate_right(63);
		v[2] = v[2].wrapping_add(m[s[6]]);
		v[2] = v[2].wrapping_add(v[6]);
		v[14] ^= v[2];
		v[14] = v[14].rotate_right(16);
		v[10] = v[10].wrapping_add(v[14]);
		v[6] ^= v[10];
		v[6] = v[6].rotate_right(63);
		v[3] = v[3].wrapping_add(m[s[7]]);
		v[3] = v[3].wrapping_add(v[7]);
		v[15] ^= v[3];
		v[15] = v[15].rotate_right(16);
		v[11] = v[11].wrapping_add(v[15]);
		v[7] ^= v[11];
		v[7] = v[7].rotate_right(63);
		v[0] = v[0].wrapping_add(m[s[8]]);
		v[0] = v[0].wrapping_add(v[5]);
		v[15] ^= v[0];
		v[15] = v[15].rotate_right(32);
		v[10] = v[10].wrapping_add(v[15]);
		v[5] ^= v[10];
		v[5] = v[5].rotate_right(24);
		v[1] = v[1].wrapping_add(m[s[9]]);
		v[1] = v[1].wrapping_add(v[6]);
		v[12] ^= v[1];
		v[12] = v[12].rotate_right(32);
		v[11] = v[11].wrapping_add(v[12]);
		v[6] ^= v[11];
		v[6] = v[6].rotate_right(24);
		v[2] = v[2].wrapping_add(m[s[10]]);
		v[2] = v[2].wrapping_add(v[7]);
		v[13] ^= v[2];
		v[13] = v[13].rotate_right(32);
		v[8] = v[8].wrapping_add(v[13]);
		v[7] ^= v[8];
		v[7] = v[7].rotate_right(24);
		v[3] = v[3].wrapping_add(m[s[11]]);
		v[3] = v[3].wrapping_add(v[4]);
		v[14] ^= v[3];
		v[14] = v[14].rotate_right(32);
		v[9] = v[9].wrapping_add(v[14]);
		v[4] ^= v[9];
		v[4] = v[4].rotate_right(24);
		v[0] = v[0].wrapping_add(m[s[12]]);
		v[0] = v[0].wrapping_add(v[5]);
		v[15] ^= v[0];
		v[15] = v[15].rotate_right(16);
		v[10] = v[10].wrapping_add(v[15]);
		v[5] ^= v[10];
		v[5] = v[5].rotate_right(63);
		v[1] = v[1].wrapping_add(m[s[13]]);
		v[1] = v[1].wrapping_add(v[6]);
		v[12] ^= v[1];
		v[12] = v[12].rotate_right(16);
		v[11] = v[11].wrapping_add(v[12]);
		v[6] ^= v[11];
		v[6] = v[6].rotate_right(63);
		v[2] = v[2].wrapping_add(m[s[14]]);
		v[2] = v[2].wrapping_add(v[7]);
		v[13] ^= v[2];
		v[13] = v[13].rotate_right(16);
		v[8] = v[8].wrapping_add(v[13]);
		v[7] ^= v[8];
		v[7] = v[7].rotate_right(63);
		v[3] = v[3].wrapping_add(m[s[15]]);
		v[3] = v[3].wrapping_add(v[4]);
		v[14] ^= v[3];
		v[14] = v[14].rotate_right(16);
		v[9] = v[9].wrapping_add(v[14]);
		v[4] ^= v[9];
		v[4] = v[4].rotate_right(63);
	}

	for i in 0..8 {
		h[i] ^= v[i] ^ v[i + 8];
	}
}


#[cfg(test)]
mod tests {
	use crate::compress;
	use rustc_hex::FromHex;

	#[test]
	fn test_blake2_f() {
		let mut h_in = [
			0x6a09e667f2bdc948_u64, 0xbb67ae8584caa73b_u64,
			0x3c6ef372fe94f82b_u64, 0xa54ff53a5f1d36f1_u64,
			0x510e527fade682d1_u64, 0x9b05688c2b3e6c1f_u64,
			0x1f83d9abfb41bd6b_u64, 0x5be0cd19137e2179_u64,
		];

		let m = [
			0x0000000000636261_u64, 0x0000000000000000_u64, 0x0000000000000000_u64,
			0x0000000000000000_u64, 0x0000000000000000_u64, 0x0000000000000000_u64,
			0x0000000000000000_u64, 0x0000000000000000_u64, 0x0000000000000000_u64,
			0x0000000000000000_u64, 0x0000000000000000_u64, 0x0000000000000000_u64,
			0x0000000000000000_u64, 0x0000000000000000_u64, 0x0000000000000000_u64,
			0x0000000000000000_u64,
		];
		let c = [3, 0];
		let f = true;
		let rounds = 12;
		let h_out: [u64; 8] = [
			0x0D4D1C983FA580BA_u64, 0xE9F6129FB697276A_u64, 0xB7C45A68142F214C_u64,
			0xD1A2FFDB6FBB124B_u64, 0x2D79AB2A39C5877D_u64, 0x95CC3345DED552C2_u64,
			0x5A92F1DBA88AD318_u64, 0x239900D4ED8623B9_u64,
		];

		compress(&mut h_in, m, c, f, rounds);

		assert_eq!(h_in, h_out);
	}

	fn to_u64_slice(vec: &[u8], slice: &mut [u64]) {
		vec.chunks(8).enumerate().for_each(|(index, val)| {
			slice[index] = u64::from_le_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]])
		})
	}

	#[test]
	fn test_vector1() {
		let vec = vec![
//			(
//				"ffffffff48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
//				"fc59093aafa9ab43daae0e914c57635c5402d8e3d2130eb9b3cc181de7f0ecf9b22bf99a7815ce16419e200e01846e6b5df8cc7703041bbceb571de6631d2615"
//			),
			(
				"0000000148c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
				"b63a380cb2897d521994a85234ee2c181b5f844d2c624c002677e9703449d2fba551b3a8333bcdf5f2f7e08993d53923de3d64fcc68c034e717b9293fed7a421"
			),
			(
				"0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000",
				"75ab69d3190a562c51aef8d88f1c2775876944407270c42c9844252c26d2875298743e7f6d5ea2f2d3e8d226039cd31b4e426ac4f2d3d666a610c2116fde4735"
			),
			(
				"0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
				"ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923"
			),
			(
				"0000000048c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
				"08c9bcf367e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d282e6ad7f520e511f6c3e2b8c68059b9442be0454267ce079217e1319cde05b"
			)
		];
		for (hex, output) in vec {
			let hex = hex;
			let bytes: Vec<u8> = hex.from_hex().unwrap();

			assert_eq!(bytes.len(), 213);

			let mut h = [0u64; 8];
			let mut m = [0u64; 16];
			let mut t = [0u64; 2];

			let rounds = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

			let f = match bytes[212] {
				1 => true,
				0 => false,
				_ => unreachable!()
			};

			to_u64_slice(&bytes[4..68], &mut h);
			to_u64_slice(&bytes[68..196], &mut m);
			to_u64_slice(&bytes[196..212], &mut t);

			compress(&mut h, m, t, f, rounds as usize);

			let output: Vec<u8> = output.from_hex().unwrap();

			let mut out = [0u64; 8];
			to_u64_slice(&output[..], &mut out);

			assert_eq!(out, h);
		}
	}
}
