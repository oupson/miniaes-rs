use consts::*;
pub use crypto_vec::*;
pub use error::*;

mod consts;
mod crypto_vec;
mod error;

pub fn encrypt_in_place<K>(buf: &mut [u8], key: K) -> Result<&mut [u8]>
where
    K: Into<CryptoVec<4>>,
{
    if buf.len() % 4 != 0 {
        return Err(Error::InvalidInputLength);
    }

    let k0 = key.into();
    let k1 = key_turn(&k0, 0);
    let k2 = key_turn(&k1, 1);

    // Turn 1
    key_add(buf, k0);
    nibble_sub(buf, &S_BOX);
    shift_row(buf);
    mix_columns(buf);

    // Turn 2
    key_add(buf, k1);
    nibble_sub(buf, &S_BOX);
    shift_row(buf);

    key_add(buf, k2);

    Ok(buf)
}

pub fn decrypt_in_place<K>(buf: &mut [u8], key: K) -> Result<&mut [u8]>
where
    K: Into<CryptoVec<4>>,
{
    if buf.len() % 4 != 0 {
        return Err(Error::InvalidInputLength);
    }

    let k0 = key.into();
    let k1 = key_turn(&k0, 0);
    let k2 = key_turn(&k1, 1);

    key_add(buf, k2);

    shift_row(buf);
    nibble_sub(buf, &S_BOX_1);
    key_add(buf, k1);

    mix_columns(buf);
    shift_row(buf);
    nibble_sub(buf, &S_BOX_1);
    key_add(buf, k0);

    Ok(buf)
}

fn key_add(buf: &mut [u8], key: CryptoVec<4>) {
    for i in 0..buf.len() {
        buf[i] ^= key[i % 4];
    }
}

fn key_turn(previous_key: &CryptoVec<4>, turn: u8) -> CryptoVec<4> {
    let mut buf = previous_key.clone();

    buf[0] = buf[0] ^ S_BOX[buf[3] as usize] ^ (0x1 << turn);
    buf[1] = buf[1] ^ buf[0];
    buf[2] = buf[2] ^ buf[1];
    buf[3] = buf[3] ^ buf[2];

    buf
}

fn nibble_sub(buf: &mut [u8], s_box: &[u8; 16]) {
    for i in 0..buf.len() {
        buf[i] = s_box[buf[i] as usize];
    }
}

fn shift_row(buf: &mut [u8]) {
    for i in (0..buf.len()).step_by(4) {
        let t = buf[i + 1];
        buf[i + 1] = buf[i + 3];
        buf[i + 3] = t;
    }
}

fn mix_columns(buf: &mut [u8]) {
    let mut new_buf = [0u8; 4];
    for i in (0..buf.len()).step_by(4) {
        new_buf[0] = gf_mul(3, buf[i + 0]) ^ gf_mul(2, buf[i + 1]);
        new_buf[1] = gf_mul(2, buf[i + 0]) ^ gf_mul(3, buf[i + 1]);
        new_buf[2] = gf_mul(3, buf[i + 2]) ^ gf_mul(2, buf[i + 3]);
        new_buf[3] = gf_mul(2, buf[i + 2]) ^ gf_mul(3, buf[i + 3]);
        buf[i..i + 4].copy_from_slice(&new_buf);
    }
}

fn gf_mul(a: u8, b: u8) -> u8 {
    GF_TABLE[a as usize][b as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_columns() {
        let mut b = [0x3, 0xF, 0xA, 0xF];
        mix_columns(&mut b);
        assert_eq!([0x8, 0x4, 0x0, 0x5], b);
    }

    #[test]
    fn encrypt_ok_with_cafe() {
        // Ok => 4f 6b mais mot de 4 bits
        let mut plaintext = [0x4, 0xf, 0x6, 0xb];
        let key = [0xC, 0xA, 0xF, 0xE];

        let ciphertext = encrypt_in_place(&mut plaintext, key).unwrap();
        assert_eq!(ciphertext, [0xB, 0x2, 0x8, 0xC]);

        let plaintext = decrypt_in_place(ciphertext, key).unwrap();
        assert_eq!(plaintext, [0x4, 0xf, 0x6, 0xb]);
    }

    #[test]
    fn decrypt() {
        let mut ciphertext = [0x3, 0x1, 0x4, 0x0];
        let key = [0x1, 0x6, 0x6, 0x4];

        let plaintext = decrypt_in_place(&mut ciphertext, key).unwrap();
        assert_eq!(plaintext, [0x7, 0x9, 0x6, 0xf]);
    }
}
