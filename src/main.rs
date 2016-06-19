// Algorithm from wikipedia:
//
// Note 1: All variables are unsigned 32-bit quantities and wrap modulo 232 when calculating, except for
// ml, the message length, which is a 64-bit quantity, and
// hh, the message digest, which is a 160-bit quantity.
// Note 2: All constants in this pseudo code are in big endian.
// Within each word, the most significant byte is stored in the leftmost byte position
//
// Initialize variables:
//
// h0 = 0x67452301
// h1 = 0xEFCDAB89
// h2 = 0x98BADCFE
// h3 = 0x10325476
// h4 = 0xC3D2E1F0
//
// ml = message length in bits (always a multiple of the number of bits in a character).
//
// Pre-processing:
// append the bit '1' to the message e.g. by adding 0x80 if message length is a multiple of 8 bits.
// append 0 ≤ k < 512 bits '0', such that the resulting message length in bits
// is congruent to −64 ≡ 448 (mod 512)
// append ml, in a 64-bit big-endian integer. Thus, the total length is a multiple of 512 bits.
//
// Process the message in successive 512-bit chunks:
// break message into 512-bit chunks
// for each chunk
// break chunk into sixteen 32-bit big-endian words w[i], 0 ≤ i ≤ 15
//
// Extend the sixteen 32-bit words into eighty 32-bit words:
// for i from 16 to 79
// w[i] = (w[i-3] xor w[i-8] xor w[i-14] xor w[i-16]) leftrotate 1
//
// Initialize hash value for this chunk:
// a = h0
// b = h1
// c = h2
// d = h3
// e = h4
//
// Main loop:[47][2]
// for i from 0 to 79
// if 0 ≤ i ≤ 19 then
// f = (b and c) or ((not b) and d)
// k = 0x5A827999
// else if 20 ≤ i ≤ 39
// f = b xor c xor d
// k = 0x6ED9EBA1
// else if 40 ≤ i ≤ 59
// f = (b and c) or (b and d) or (c and d)
// k = 0x8F1BBCDC
// else if 60 ≤ i ≤ 79
// f = b xor c xor d
// k = 0xCA62C1D6
//
// temp = (a leftrotate 5) + f + e + k + w[i]
// e = d
// d = c
// c = b leftrotate 30
// b = a
// a = temp
//
// Add this chunk's hash to result so far:
// h0 = h0 + a
// h1 = h1 + b
// h2 = h2 + c
// h3 = h3 + d
// h4 = h4 + e
//
// Produce the final hash value (big-endian) as a 160 bit number:
// hh = (h0 leftshift 128) or (h1 leftshift 96) or (h2 leftshift 64) or (h3 leftshift 32) or h4
//

fn main() {}

fn hex_str(input: &str) -> &str {
    let bytes = input.as_bytes();
    let sha = compute_sha(bytes);
    return "hi";
}

fn u8s_to_u32(u8s: &[u8]) -> u32 {
    ((u8s[0] as u32) << 24) | ((u8s[1] as u32) << 16) | ((u8s[2] as u32) << 8) | (u8s[3] as u32)
}

fn compute_sha(input: &[u8]) -> [u32; 5] {
    let mut message = input.to_vec();
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    pre_process_message(&mut message);

    let chunk_size = 512;
    let blocks_per_chunk = chunk_size / 8;
    let block_count = message.len() / blocks_per_chunk;
    let chunk_start_indexes = (0..block_count).map(|x| x * blocks_per_chunk);

    for chunk_start in chunk_start_indexes {
        let mut w: [u32; 80] = [0; 80];
        for i in 0..16 {
            let slice_start = chunk_start + (i * 4);
            let slice_end = slice_start + 4;
            let int_slice = &message[slice_start..slice_end];
            w[i] = u8s_to_u32(int_slice);
        }

        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for i in 0..80 {
            let (f, k) = match i {
                0...19 => ((b & c) | ((!b) & d), 0x5A827999),
                20...39 => ((b ^ c ^ d), 0x6ED9EBA1),
                40...59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC), 
                60...79 => ((b ^ c ^ d), 0xCA62C1D6),
                _ => panic!(),
            };

            let temp = a.rotate_left(5)
                .overflowing_add(f)
                .0
                .overflowing_add(e)
                .0
                .overflowing_add(k)
                .0
                .overflowing_add(w[i])
                .0;
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.overflowing_add(a).0;
        h1 = h1.overflowing_add(b).0;
        h2 = h2.overflowing_add(c).0;
        h3 = h3.overflowing_add(d).0;
        h4 = h4.overflowing_add(e).0;
    }

    [h0, h1, h2, h3, h4]
}


fn pre_process_message(message: &mut Vec<u8>) {
    // Append a 1 bit to message, and pad with 0s until final 64-bits which
    // should be the original message length.  The resulting message should be
    // eavenly diviable in to 512 bit chunks.

    let chunk_size = 512;
    let blocks = chunk_size / 8;
    let blocks_at_end_for_len = 64 / 8;
    let max_pad = blocks - blocks_at_end_for_len;
    let one_bit: u8 = 0b10000000;
    let len: u64 = (8 * message.len()) as u64;

    message.push(one_bit);

    let pad_count = (64 - ((message.len() + 8) % 64)) % 64;
    for i in 0..pad_count {
        message.push(0);
    }

    for i in (0..8).rev() {
        message.push((len >> i * 8) as u8);
    }
}


#[test]
fn known_sha() {
    let string = "The quick brown fox jumps over the lazy dog".as_bytes();
    let expected = [
        0x2fd4e1c6,
        0x7a2d28fc,
        0xed849ee1,
        0xbb76e739,
        0x1b93eb12,
    ];

    let sha = compute_sha(&string);
    assert_eq!(expected, sha);
}

#[test]
fn blank_sha() {
    let sha = compute_sha(&[]);
    let h0 = 0xda39a3ee;
    let h1 = 0x5e6b4b0d;
    let h2 = 0x3255bfef;
    let h3 = 0x95601890;
    let h4 = 0xafd80709;

    let expected = [h0, h1, h2, h3, h4];

    assert_eq!(expected, sha);
}

#[test]
fn pre_process_length_min_512_bits() {
    let mut msg: Vec<u8> = vec![4];
    pre_process_message(&mut msg);
    assert_eq!(msg.len(), 64);
}

#[test]
fn pre_process_exactly_512_bits() {
    let mut msg: Vec<u8> = vec![1; 55];
    pre_process_message(&mut msg);
    assert_eq!(msg.len(), 64);
}

#[test]
fn pre_process_multiple_512_bits() {
    let mut msg: Vec<u8> = vec![1; 64];
    pre_process_message(&mut msg);
    assert_eq!(msg.len(), 128);
}

#[test]
fn pre_process_msg_len_appended() {
    let mut msg: Vec<u8> = vec![1; 4];
    pre_process_message(&mut msg);

    // Last 64 bits should be input message length
    assert_eq!(Some(&32), msg.last());
}
