use std::{
    ops::{Add, Sub},
    iter::Sum,
    fs::File,
    io::Write,
    path::Path,
};
use num::PrimInt;
use sodiumoxide::crypto::{
    sign, 
    sign::ed25519
    };
use zksnark::Proof;
use serde_derive::{Serialize, Deserialize};
use serde::{Serialize as Ser, Deserialize as De};
use serde_json::{to_string, from_str, from_reader};

#[derive(Serialize, Deserialize)]
pub struct SnowGlobe<V, W, U> {
    prf: Proof<V, W>,
    pk: ed25519::PublicKey,
    sig: ed25519::Signature,
    ver: Vec<U>,
}

impl<'de, V, W, U> SnowGlobe<V, W, U> 
where 
    V: Add<Output=V> 
        + Sub<Output=V>
        + Sum 
        + Copy 
        + Ser 
        + De<'de>,
    W: Add<Output=W> 
        + Sum 
        + Copy 
        + Ser 
        + De<'de>,
    U: PrimInt,
{
    pub fn sign_message_from_file<T: Ser>(
        m: T, pk_path: &'static str, sk_path: &'static str
    ) -> (ed25519::Signature, ed25519::PublicKey) {
        match File::open(sk_path) {
            Ok(_) => {
                let (pk, sk) = Self::read_key_from_file(pk_path, sk_path);
                return (
                    sign::sign_detached(
                        to_string(&m).expect("SnowGlobe::sign_proof() parsing &m as String").as_bytes(), 
                        &sk
                    ),
                    pk
                )
            },
            Err(_) => {
                let (pk, sk) = Self::write_key_to_file(pk_path, sk_path);
                return (
                    sign::sign_detached(
                        to_string(&m).expect("SnowGlobe::sign_proof() parsing &m as String").as_bytes(), 
                        &sk
                    ),
                    pk
                )
            },
        }
    }

    // consider from_utf8 as we are reading bytes.
    pub fn read_key_from_file(pk_path: &'static str, sk_path: &'static str) -> (ed25519::PublicKey, ed25519::SecretKey) {
        let pk: ed25519::PublicKey = match File::open(pk_path) {
            Ok(file) => from_reader(file).expect("SnowGlobe::read_key_from_file() reading Ed25519 Public Key from File"),
            Err(e) => panic!("SnowGlobe::read_key_from_file() opening File for Ed25519 Public Key: {}", e), 
        };
        let sk: ed25519::SecretKey = match File::open(sk_path) {
            Ok(file) => from_reader(file).expect("SnowGlobe::read_key_from_file() reading Ed25519 Secret Key from File"),
            Err(e) => panic!("SnowGlobe::read_key_from_file() opening File for Ed25519 Secret Key: {}", e), 
        };
        (pk, sk)
    }

    pub fn write_key_to_file(pk_path: &'static str, sk_path: &'static str) -> (ed25519::PublicKey, ed25519::SecretKey) {
        let (pk, sk) = sign::gen_keypair();
        println!("{:?}", &pk);
        match File::create(
            Path::new(pk_path)
        ) {
            Ok(mut x) => {
                x.write_all(
                    pk.as_ref()
                ).expect("SnowGlobe::write_key() writing Ed25519 Public Key to File");
            },
            Err(e) => panic!("SnowGlobe::write_key() creating File for Ed25519 Public Key: {}", e),
        }
        match File::create(
            Path::new(sk_path)
        ) {
            Ok(mut x) => {
                x.write_all(
                    sk.as_ref()
                ).expect("SnowGlobe::write_key() writing Ed25519 Secret Key to File");
            },
            Err(e) => panic!("SnowGlobe::write_key() creating File for Ed25519 Secret Key: {}", e),
        }
        (pk, sk)
    }

}    

#[test]
fn test_ed25519_from_file() {
    use zksnark::groth16::fr::{G1Local, G2Local};

    let pk_path = "src/tests/files/signature/ed25519_from_file/ed25519_public.key";
    let sk_path = "src/tests/files/signature/ed25519_from_file/ed25519_secret.key";
    let data = b"test_signature";
    let (sig, pk) = SnowGlobe::<G1Local, G2Local, usize>::sign_message_from_file(data, pk_path, sk_path);
    println!("{} {}", to_string(&sig).unwrap(), to_string(&pk).unwrap());
    assert!(sign::verify_detached(&sig, data, &pk));
}