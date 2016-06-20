use std::io;
use std::io::Read;

fn main() {
    let mut buf: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut buf).expect("Error");
    let digest = compute_sha(&mut buf);
    println!("{}", format_digest(&digest));
}

fn format_digest(digest: &[u32; 5]) -> String {
    format!("{:08x}{:08x}{:08x}{:08x}{:08x}",
            digest[0],
            digest[1],
            digest[2],
            digest[3],
            digest[4])
}

fn u8s_to_u32(u8s: &[u8]) -> u32 {
    ((u8s[0] as u32) << 24) | ((u8s[1] as u32) << 16) | ((u8s[2] as u32) << 8) | (u8s[3] as u32)
}

fn compute_sha(mut message: &mut Vec<u8>) -> [u32; 5] {
    // Algorithm from wikipedia: https://en.wikipedia.org/wiki/SHA-1
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

// Append a 1 bit to message, and pad with 0s until final 64-bits which
// should be the original message length.  The resulting message should be
// eavenly diviable in to 512 bit chunks.
fn pre_process_message(message: &mut Vec<u8>) {
    let one_bit: u8 = 0b10000000;
    let len: u64 = (8 * message.len()) as u64;

    message.push(one_bit);

    let pad_count = (64 - ((message.len() + 8) % 64)) % 64;
    for _ in 0..pad_count {
        message.push(0);
    }

    for i in (0..8).rev() {
        message.push((len >> i * 8) as u8);
    }
}

#[test]
fn known_sha() {
    let mut string = "The quick brown fox jumps over the lazy dog".as_bytes().to_vec();
    let expected = [0x2fd4e1c6, 0x7a2d28fc, 0xed849ee1, 0xbb76e739, 0x1b93eb12];

    let sha = compute_sha(&mut string);
    assert_eq!(expected, sha);
}

#[test]
fn blank_sha() {
    let sha = compute_sha(&mut [].to_vec());
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
