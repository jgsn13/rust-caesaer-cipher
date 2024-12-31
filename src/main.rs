fn caesear_encrypt(plain_text: String, alphabet: &str, key: usize) -> String {
    let mut cipher_text = String::from("");
    for plain_char in plain_text.to_uppercase().chars().into_iter() {
        match alphabet.find(plain_char) {
            Some(idx) => {
                let cipher_index = (idx + key) % alphabet.len();
                let cipher_char = alphabet.chars().nth(cipher_index).unwrap();
                cipher_text.push(cipher_char);
            }
            None => {
                let cipher_char = alphabet.chars().nth(alphabet.len() - 1).unwrap();
                cipher_text.push(cipher_char);
            }
        };
    }

    cipher_text
}

fn caesear_decrypt(cipher_text: String, alphabet: &str, key: usize) -> String {
    let mut plain_text = String::from("");
    for cipher_char in cipher_text.to_uppercase().chars().into_iter() {
        match alphabet.find(cipher_char) {
            Some(idx) => {
                let plain_index = idx
                    .checked_sub(key)
                    .unwrap_or(alphabet.len() - key + idx) // handle subtract overflow
                    % alphabet.len();
                let plain_char = alphabet.chars().nth(plain_index).unwrap();
                plain_text.push(plain_char);
            }
            None => {
                let cipher_char = alphabet.chars().nth(alphabet.len() - 1).unwrap();
                plain_text.push(cipher_char);
            }
        };
    }

    plain_text
}

fn crack_caesar(cipher_text: String, alphabet: &str) {
    for key in 0..alphabet.len() {
        let mut plain_text = String::from("");
        for c in cipher_text.to_uppercase().chars().into_iter() {
            match alphabet.find(c) {
                Some(idx) => {
                    let plain_index =
                        idx.checked_sub(key).unwrap_or(alphabet.len() - key + idx) % alphabet.len();
                    let plain_char = alphabet.chars().nth(plain_index).unwrap();
                    plain_text.push(plain_char);
                }
                None => {
                    let cipher_char = alphabet.chars().nth(alphabet.len() - 1).unwrap();
                    plain_text.push(cipher_char);
                }
            };
        }
        println!("With key {}, the result is: {}", key, plain_text);
    }
}

fn main() {
    const ALPHABET: &str = " !ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const KEY: u8 = 3;

    let message = String::from("My name is Joaquim Gregorio!");
    let encrypted_message = caesear_encrypt(message, ALPHABET, KEY as usize);
    println!("Encrypted Message: {}", encrypted_message);
    // println!("Decrypted Message: {}", caesear_decrypt(encrypted_message, ALPHABET, KEY as usize));
    crack_caesar(encrypted_message, ALPHABET);
}
