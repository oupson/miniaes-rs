use miniaes::encrypt_in_place;

fn main() {
    encrypt();
}

fn encrypt() {
    let msg = "B, A, BA. Compris."
        .chars()
        .map(|c| c as u8)
        .flat_map(|c| [(c & 0b1111000) >> 4, c & 0b00001111])
        .collect::<Vec<u8>>();

    let mut res = Vec::new();

    let key = [0xb, 0x3, 0x3, 0xf];
    let mut iv = [1, 0, 0, 7];
    for block in msg.chunks(4) {
        println!("{:02X?}", block);
        let mut to_encrypt = iv.clone();
        for i in 0..4 {
            to_encrypt[i] ^= block[i];
        }
        encrypt_in_place(&mut to_encrypt, key.clone()).unwrap();
        res.extend_from_slice(&to_encrypt);
        iv = to_encrypt;
    }

    let p2: Vec<u8> = res.chunks(2).map(|a| (a[0] << 4) + a[1]).collect();
    println!("{:02X?}", p2);
}
