use miniaes::decrypt_in_place;

const MSG: &'static str = include_str!("msg.txt");

fn main() {
    let key = [0xf, 0xe, 0xe, 0x7];
    let mut ciphertext: Vec<u8> = MSG
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect();

    let plaintext = decrypt_in_place(&mut ciphertext, key).expect("Failed to decrypt");

    let p2: Vec<u8> = plaintext.chunks(2).map(|a| (a[0] << 4) + a[1]).collect();

    println!("{}", String::from_utf8_lossy(&p2).to_string());
}
