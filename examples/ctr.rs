fn main() {
    let mut ciphertext: Vec<u8> = "4e5a f05d 915f 572b da66 006d 8326"
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect();

    let a = 0x5eed;
    let key = [0xb, 0x3, 0x3, 0xf];

    let res = ciphertext
        .chunks_mut(4)
        .enumerate()
        .flat_map(|(i, c)| {
            let i = a + i;
            let mut block = [
                (i >> 12 & 0b1111) as u8,
                (i >> 8 & 0b1111) as u8,
                (i >> 4 & 0b1111) as u8,
                (i >> 0 & 0b1111) as u8,
            ];
            miniaes::encrypt_in_place(&mut block, key.clone()).unwrap();

            for i in 0..4 {
                c[i] ^= block[i];
            }

            c.chunks(2).map(|a| (a[0] << 4) + a[1]).collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", String::from_utf8(res))
}
