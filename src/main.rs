const ALPHABET: &str = " !ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const KEY: u8 = 3;

fn caesear_encrypt(plain_text: String) -> String {
    let mut cipher_text = String::from("");
    for plain_char in plain_text.to_uppercase().chars().into_iter() {
        match ALPHABET.find(plain_char) {
            Some(idx) => {
                let cipher_index = (idx + (KEY as usize)) % ALPHABET.len();
                let cipher_char = ALPHABET.chars().nth(cipher_index).unwrap();
                cipher_text.push(cipher_char);
            }
            None => {
                let cipher_char = ALPHABET.chars().nth(ALPHABET.len() - 1).unwrap();
                cipher_text.push(cipher_char);
            }
        };
    }

    cipher_text
}

fn caesear_decrypt(cipher_text: String) -> String {
    let mut plain_text = String::from("");
    for cipher_char in cipher_text.to_uppercase().chars().into_iter() {
        match ALPHABET.find(cipher_char) {
            Some(idx) => {
                let plain_index = idx
                    .checked_sub(KEY as usize)
                    .unwrap_or(ALPHABET.len() - (KEY as usize) + idx) // handle subtract overflow
                    % ALPHABET.len();
                let plain_char = ALPHABET.chars().nth(plain_index).unwrap();
                plain_text.push(plain_char);
            }
            None => {
                let cipher_char = ALPHABET.chars().nth(ALPHABET.len() - 1).unwrap();
                plain_text.push(cipher_char);
            }
        };
    }

    plain_text
}

fn main() {
    let message = String::from("My name is Joaquim Gregorio!");
    let encrypted_message = caesear_encrypt(message);
    println!("Encrypted Message: {}", encrypted_message);
    println!("Decrypted Message: {}", caesear_decrypt(encrypted_message));
}
