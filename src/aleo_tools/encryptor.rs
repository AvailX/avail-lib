use once_cell::sync::OnceCell;
use snarkvm::{circuit::prelude::IndexMap, prelude::*};
pub struct Encryptor<N: Network> {
    _phantom: std::marker::PhantomData<N>,
}

impl<N: Network> Encryptor<N> {
    /// Encrypt a private key into ciphertext using a secret
    pub fn encrypt_private_key_with_secret(
        private_key: &PrivateKey<N>,
        secret: &str,
    ) -> Result<Ciphertext<N>> {
        Self::encrypt_field(&private_key.seed(), secret, "private_key")
    }

    /// Decrypt a private key from ciphertext using a secret
    pub fn decrypt_private_key_with_secret(
        ciphertext: &Ciphertext<N>,
        secret: &str,
    ) -> Result<PrivateKey<N>> {
        let seed = Self::decrypt_field(ciphertext, secret, "private_key")?;
        PrivateKey::try_from(seed)
    }

    /// Encrypt a view key into ciphertext using a secret
    pub fn encrypt_view_key_with_secret(
        view_key: &ViewKey<N>,
        secret: &str,
    ) -> Result<Ciphertext<N>> {
        let view_key_field = match view_key.to_field() {
            Ok(field) => field,
            Err(_) => return Err(anyhow!("Error converting view key to field")),
        };

        Self::encrypt_field(&view_key_field, secret, "view_key")
    }

    /// Decrypt a view key from ciphertext using a secret
    pub fn decrypt_view_key_with_secret(
        ciphertext: &Ciphertext<N>,
        secret: &str,
    ) -> Result<ViewKey<N>> {
        let view_key_field = Self::decrypt_field(ciphertext, secret, "view_key")?;
        let view_key = ViewKey::<N>::from_bytes_le(&view_key_field.to_bytes_le()?)?;

        Ok(view_key)
    }

    // Encrypted a field element into a ciphertext representation
    fn encrypt_field(field: &Field<N>, secret: &str, domain: &str) -> Result<Ciphertext<N>> {
        // Derive the domain separators and the secret.
        let domain = Field::<N>::new_domain_separator(domain);
        let secret = Field::<N>::new_domain_separator(secret);

        // Generate a nonce
        let mut rng = rand::thread_rng();
        let nonce = Uniform::rand(&mut rng);

        // Derive a blinding factor and create an encryption target
        let blinding = N::hash_psd2(&[domain, nonce, secret])?;
        let key = blinding * field;
        let plaintext = Plaintext::<N>::Struct(
            IndexMap::from_iter(vec![
                (
                    Identifier::from_str("key")?,
                    Plaintext::<N>::from(Literal::Field(key)),
                ),
                (
                    Identifier::from_str("nonce")?,
                    Plaintext::<N>::from(Literal::Field(nonce)),
                ),
            ]),
            OnceCell::new(),
        );
        plaintext.encrypt_symmetric(secret)
    }

    // Recover a field element encrypted within ciphertext
    fn decrypt_field(ciphertext: &Ciphertext<N>, secret: &str, domain: &str) -> Result<Field<N>> {
        let domain = Field::<N>::new_domain_separator(domain);
        let secret = Field::<N>::new_domain_separator(secret);
        let decrypted = ciphertext.decrypt_symmetric(secret)?;
        let recovered_key = Self::extract_value(&decrypted, "key")?;
        let recovered_nonce = Self::extract_value(&decrypted, "nonce")?;
        let recovered_blinding = N::hash_psd2(&[domain, recovered_nonce, secret])?;
        Ok(recovered_key / recovered_blinding)
    }

    // Extract a field element from a plaintext
    fn extract_value(plaintext: &Plaintext<N>, identifier: &str) -> Result<Field<N>> {
        let identity = Identifier::from_str(identifier)?;
        let value = plaintext.find(&[identity])?;
        match value {
            Plaintext::<N>::Literal(literal, ..) => match literal {
                Literal::Field(recovered_value) => Ok(recovered_value),
                _ => Err(anyhow!("Wrong literal type")),
            },
            _ => Err(anyhow!("Expected literal")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use snarkvm::console::{network::Testnet3 as CurrentNetwork, prelude::TestRng};
    /* View Key Tests */

    /* Private Key Test */
    #[test]
    fn test_encryptor_encrypt_and_decrypt() {
        let mut rng = TestRng::default();
        let private_key = PrivateKey::<CurrentNetwork>::new(&mut rng).unwrap();
        let enc = Encryptor::<CurrentNetwork>::encrypt_private_key_with_secret(
            &private_key,
            "mypassword",
        )
        .unwrap();
        let recovered_private_key =
            Encryptor::<CurrentNetwork>::decrypt_private_key_with_secret(&enc, "mypassword")
                .unwrap();
        assert_eq!(private_key, recovered_private_key);
    }

    #[test]
    fn test_encryptor_wrong_private_key_doesnt_decrypt() {
        let mut rng = TestRng::default();
        let private_key = PrivateKey::<CurrentNetwork>::new(&mut rng).unwrap();
        let enc = Encryptor::<CurrentNetwork>::encrypt_private_key_with_secret(
            &private_key,
            "mypassword",
        )
        .unwrap();
        let recovered_private_key =
            Encryptor::<CurrentNetwork>::decrypt_private_key_with_secret(&enc, "wrong_password");
        assert!(recovered_private_key.is_err())
    }

    #[test]
    fn test_encryptor_same_secret_doesnt_produce_same_ciphertext_on_different_runs() {
        let mut rng = TestRng::default();
        let private_key = PrivateKey::<CurrentNetwork>::new(&mut rng).unwrap();
        let enc = Encryptor::encrypt_private_key_with_secret(&private_key, "mypassword").unwrap();
        let enc2 = Encryptor::encrypt_private_key_with_secret(&private_key, "mypassword").unwrap();
        assert_ne!(enc, enc2);

        // Assert that we can decrypt both ciphertexts with the same secret despite being different
        let recovered_key_1 =
            Encryptor::decrypt_private_key_with_secret(&enc, "mypassword").unwrap();
        let recovered_key_2 =
            Encryptor::decrypt_private_key_with_secret(&enc, "mypassword").unwrap();
        assert_eq!(recovered_key_1, recovered_key_2);
    }

    #[test]
    fn test_encryptor_private_keys_encrypted_with_different_passwords_match() {
        let mut rng = TestRng::default();
        let private_key = PrivateKey::<CurrentNetwork>::new(&mut rng).unwrap();
        let enc = Encryptor::encrypt_private_key_with_secret(&private_key, "mypassword").unwrap();
        let enc2 = Encryptor::encrypt_private_key_with_secret(&private_key, "mypassword2").unwrap();
        assert_ne!(enc, enc2);

        // Assert that we can decrypt both ciphertexts with to the same key
        let recovered_key_1 =
            Encryptor::decrypt_private_key_with_secret(&enc, "mypassword").unwrap();
        let recovered_key_2 =
            Encryptor::decrypt_private_key_with_secret(&enc2, "mypassword2").unwrap();
        assert_eq!(recovered_key_1, recovered_key_2);
    }

    #[test]
    fn test_encryptor_different_private_keys_encrypted_with_same_password_dont_match() {
        let mut rng = TestRng::default();
        let private_key = PrivateKey::<CurrentNetwork>::new(&mut rng).unwrap();
        let private_key2 = PrivateKey::<CurrentNetwork>::new(&mut rng).unwrap();
        let enc = Encryptor::encrypt_private_key_with_secret(&private_key, "mypassword").unwrap();
        let enc2 = Encryptor::encrypt_private_key_with_secret(&private_key2, "mypassword").unwrap();
        assert_ne!(enc, enc2);

        // Assert that private key plaintexts don't match
        let recovered_key_1 =
            Encryptor::decrypt_private_key_with_secret(&enc, "mypassword").unwrap();
        let recovered_key_2 =
            Encryptor::decrypt_private_key_with_secret(&enc2, "mypassword").unwrap();
        assert_ne!(recovered_key_1, recovered_key_2);
    }
}
