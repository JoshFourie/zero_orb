use ring::{
    rand,
    signature, 
    signature::KeyPair,
    signature::Ed25519KeyPair,
};

// Ring API has adjusted for the most recent update and the method public_key_bytes() is apparently deprecated,
// alternatively, call the public_key() method on a KeyPair held in the KeyPair trait imported above.
// This impl is valid for Ring 0.14.0.

pub trait SignatureScheme {
    type Signature;
    type KeyPair;
    type PublicKey;

    fn sign_message(&self, key: &Self::KeyPair) -> Self::Signature;
    fn verify_signature(&self, sig: &Vec<u8>, key: &Self::PublicKey) -> bool;
    fn public_key(key: &Self::KeyPair) -> Self::PublicKey;
    fn init_key_pair() -> Self::KeyPair;
}

pub struct EdDSA<T>(T);

impl<T> EdDSA<T> 
where
    T: AsRef<[u8]>
{
    pub fn into(s: T) -> Self {
        EdDSA(s)
    }
}

impl<T> SignatureScheme for EdDSA<T> 
where
    T: AsRef<[u8]>
{
    type Signature = signature::Signature;
    type KeyPair = signature::Ed25519KeyPair;
    type PublicKey = Vec<u8>;

    fn sign_message(&self, key: &Self::KeyPair) -> Self::Signature {
        key.sign(&self.0.as_ref())
    }
    fn verify_signature(&self, sig: &Vec<u8>, key: &Self::PublicKey) -> bool {
        match signature::verify(
            &signature::ED25519, 
            untrusted::Input::from(
                key
            ),
            untrusted::Input::from(
                &self.0.as_ref()
            ), 
            untrusted::Input::from(
                sig.as_ref()
            ),
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    fn public_key(key: &Self::KeyPair) -> Self::PublicKey {
        key.public_key()
            .as_ref()
            .to_vec()
    }
    fn init_key_pair() -> Self::KeyPair {
        Ed25519KeyPair::from_pkcs8(
            untrusted::Input::from(
                Ed25519KeyPair::generate_pkcs8(&rand::SystemRandom::new())
                    .expect("String::CryoKey::init_key_pair() panicked at the associated function: generate_pkcs8()")
                .as_ref()
            )
        )
        .expect("String::CryoKey::init_key_pair() panicked at the associated function: from_pkcs8")
    }    
}

#[cfg(test)]
mod tests {
    use crate::crypto::{EdDSA, SignatureScheme};

    #[test]
    fn test_ed_key() {
        let kp = EdDSA::<String>::init_key_pair();
        let sig = EdDSA::into("testing_cryo_key")
            .sign_message(&kp);
        assert!(EdDSA::into("testing_cryo_key")
            .verify_signature(
                &sig
                    .as_ref()
                    .to_vec(),
                &EdDSA::<String>::public_key(&kp),
            )
        );
    }
}


