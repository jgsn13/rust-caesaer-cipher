struct CaesarCipher {
    alphabet: String,
    key: usize,
}

impl CaesarCipher {
    fn new(alphabet: String, key: usize) -> CaesarCipher {
        CaesarCipher { alphabet, key }
    }

    fn encrypt(&self, plain_text: &String) -> String {
        let mut cipher_text = String::from("");
        for character in plain_text.to_uppercase().chars().into_iter() {
            let idx = self.alphabet.find(character).unwrap();
            let cipher_index = (idx + self.key) % self.alphabet.len();
            let cipher_char = self.alphabet.chars().nth(cipher_index).unwrap();
            cipher_text.push(cipher_char);
        }
        cipher_text
    }

    fn decrypt(&self, cipher_text: &String) -> String {
        let mut plain_text = String::from("");
        for character in cipher_text.to_uppercase().chars().into_iter() {
            let idx = self.alphabet.find(character).unwrap();
            let plain_index = idx
                .checked_sub(self.key)
                .unwrap_or(self.alphabet.len() - self.key + idx) // handle subtract overflow
                % self.alphabet.len();
            let plain_char = self.alphabet.chars().nth(plain_index).unwrap();
            plain_text.push(plain_char);
        }
        plain_text
    }

    fn crack(&self, cipher_text: String) {
        for key in 0..(self.alphabet.len()) {
            let mut plain_text = String::from("");
            for character in cipher_text.to_uppercase().chars().into_iter() {
                let idx = self.alphabet.find(character).unwrap();
                let plain_index = idx
                    .checked_sub(key)
                    .unwrap_or(self.alphabet.len() - key + idx)
                    % self.alphabet.len();
                let plain_char = self.alphabet.chars().nth(plain_index).unwrap();
                plain_text.push(plain_char);
            }
            println!("With key {key}, the result is: {plain_text}");
        }
    }
}

fn main() {
    let alphabet = String::from(" !ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = 3;
    let caesar = CaesarCipher::new(alphabet, key);

    let message = String::from("My name is Joaquim Gregorio!");
    let encrypted_message = caesar.encrypt(&message);
    let decrypted_message = caesar.decrypt(&encrypted_message);
    println!("Message: {message}\n");
    println!("Encrypted Message: {encrypted_message}");
    println!("Decrypted Message: {decrypted_message}\n");

    caesar.crack(encrypted_message);
}
