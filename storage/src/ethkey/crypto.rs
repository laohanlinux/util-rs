use secp256k1;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Err code: {}", _0)]
    Secp(secp256k1::Error),
    #[fail(display = "Err code: {}", _0)]
    Io(::std::io::Error),
    #[fail(display = "An error has occurred.")]
    InvalidMessage,
    #[fail(display = "symm error has occurred.")]
    Symm,
}

pub mod sign {
    use common;
    use ::crypto::Hash;
    use ethkey::{Secret, Public, Generator, SECP256K1, Error};
    use secp256k1::{self, key, Secp256k1, Signature, Message, ContextFlag};

    pub fn verify(public: &Public, sign: &Signature, plain_text_hash: &Hash) -> bool {
//        let plain_text_hash = common::to_keccak(plain_text);
        let context = &SECP256K1;
        /// the first byte flag whether compress
        let pdata = {
            let mut temp = [4u8; 65];
            (&mut temp[1..65]).copy_from_slice(&public[0..64]);
            temp
        };
        let publ = key::PublicKey::from_slice(context, &pdata).unwrap();
        context.verify(
            &Message::from_slice(plain_text_hash.as_ref()).unwrap(),
            &sign,
            &publ,
        ).is_ok()
    }

    pub fn sign(message: &Message, secret: &Secret) -> Signature {
        let context = &SECP256K1;
        context.sign(message, &secret.to_secp256k1_secret().unwrap()).unwrap()
    }
}


/// ECDH
//pub mod ecdh {
//    use secp256k1::{self, ecdh, key};
//    use super::Error;
//    use ethkey::{Secret, Public, SECP256K1};
//
//    /// Agree on a shared secret
//    pub fn agree(secret: &Secret, pubblic: &Public) -> Result<Secret, Error> {
//        let context = &SECP256K1;
//        /// the first byte flag whether compress
//        let pdata = {
//            let mut temp = [4u8; 65];
//            (&mut temp[1..65]).copy_from_slice(&pubblic[0..64]);
//            temp;
//        };
//
//        let publ = key::PublicKey::from_slice(context, &pdata);
//        let sec = key::SecretKey::from_slice(context, &secret);
//        let shared = ecdh::SharedSecret::new(context, &publ, &secret);
//
//        Secret::from_unsafe_slice(&shared[0..32])
//            .map_err(|_| Error::Secp(secp256k1::Error::InvalidSecretKey))
//    }
//}
//
///// ECIES function
//pub mod ecies{
//    use ethereum_types::H128;
//    use parity_crypto::{aes, digest, hmac, is_equal};
//    use ethkey::{Generator, Public, Secret, random::Random};
//    use super::{ecdh, Error};
//
//    /// Encrypt a message with a public key, writinh an HMAC covering both
//    /// the plaintext and authenticated data.
//    ///
//    /// Authenticated data may be empty.
//    	/// Encrypt a message with a public key, writing an HMAC covering both
//	/// the plaintext and authenticated data.
//	///
//	/// Authenticated data may be empty.
//    pub fn encrypt(public: &Public, auth_data: &[u8], plain: &[u8]) -> Result<Vec<u8>, Error> {
//        let r = Random.generate()?;
//        let z = ecdh::agree(r.secret(), public)?;
//        let mut key = [0u8; 32];
//        kdf(&z, &[0u8; 0], &mut key);
//
//        let ekey = &key[0..16];
//        let mkey = hmac::SigKey::sha256(&digest::sha256(&key[16..32]));
//
//        let mut msg = vec![0u8; 1 + 64 + 16 + plain.len() + 32];
//        msg[0] = 0x04u8;
//        {
//            let msgd = &mut msg[1..];
//            msgd[0..64].copy_from_slice(r.public());
//            let iv = H128::random();
//            msgd[64..80].copy_from_slice(&iv);
//            {
//                let cipher = &mut msgd[(64 + 16)..(64 + 16 + plain.len())];
//                aes::encrypt_128_ctr(ekey, &iv, plain, cipher)?;
//            }
//            let mut hmac = hmac::Signer::with(&mkey);
//            {
//                let cipher_iv = &msgd[64..(64 + 16 + plain.len())];
//                hmac.update(cipher_iv);
//            }
//            hmac.update(auth_data);
//            let sig = hmac.sign();
//            msgd[(64 + 16 + plain.len())..].copy_from_slice(&sig);
//        }
//        Ok(msg)
//    }
//
//    /// Decrypt a message with a secret key, checking HMAC for ciphertext
//    /// and authenticated data validity.
//    pub fn decrypt(secret: &Secret, auth_data: &[u8], encrypted: &[u8]) -> Result<Vec<u8>, Error> {
//        let meta_len = 1 + 64 + 16 + 32;
//        if encrypted.len() < meta_len  || encrypted[0] < 2 || encrypted[0] > 4 {
//            return Err(Error::InvalidMessage); //invalid message: publickey
//        }
//
//        let e = &encrypted[1..];
//        let p = Public::from_slice(&e[0..64]);
//        let z = ecdh::agree(secret, &p)?;
//        let mut key = [0u8; 32];
//        kdf(&z, &[0u8; 0], &mut key);
//
//        let ekey = &key[0..16];
//        let mkey = hmac::SigKey::sha256(&digest::sha256(&key[16..32]));
//
//        let clen = encrypted.len() - meta_len;
//        let cipher_with_iv = &e[64..(64+16+clen)];
//        let cipher_iv = &cipher_with_iv[0..16];
//        let cipher_no_iv = &cipher_with_iv[16..];
//        let msg_mac = &e[(64+16+clen)..];
//
//        // Verify tag
//        let mut hmac = hmac::Signer::with(&mkey);
//        hmac.update(cipher_with_iv);
//        hmac.update(auth_data);
//        let mac = hmac.sign();
//
//        if !is_equal(&mac.as_ref()[..], msg_mac) {
//            return Err(Error::InvalidMessage);
//        }
//
//        let mut msg = vec![0u8; clen];
//        aes::decrypt_128_ctr(ekey, cipher_iv, cipher_no_iv, &mut msg[..])?;
//        Ok(msg)
//    }
//
//    fn kdf(secret: &Secret, s1: &[u8], dest: &mut [u8]) {
//        // SEC/ISO/Shoup specify counter size SHOULD be equivalent
//        // to size of hash output, however, it also notes that
//        // the 4 bytes is okay. NIST specifies 4 bytes.
//        let mut ctr = 1u32;
//        let mut written = 0usize;
//        while written < dest.len() {
//            let mut hasher = digest::Hasher::sha256();
//            let ctrs = [(ctr >> 24) as u8, (ctr >> 16) as u8, (ctr >> 8) as u8, ctr as u8];
//            hasher.update(&ctrs);
//            hasher.update(secret);
//            hasher.update(s1);
//            let d = hasher.finish();
//            &mut dest[written..(written + 32)].copy_from_slice(&d);
//            written += 32;
//            ctr += 1;
//        }
//    }
//}
#[cfg(test)]
mod test {
    use super::*;
    use ::crypto::{CryptoHash, Hash, hash};
    use ::encoding;
    use ethereum_types::H256;
    use ethkey::{Public, Address, KeyPair};
    use ethkey::Generator;
    use ethkey::random::Random;
    use std::io::{self, Write};
    use serde::{Serialize, Deserialize};
    use rmps::{Serializer, Deserializer};

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Block {
        height: u64,
        validator: Vec<Validator>,
    }

    implement_cryptohash_traits! {Block}

    impl Block {
        fn new(height: u64, validator: Vec<Validator>) -> Block {
            Block {
                height,
                validator,
            }
        }
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Validator {
        address: Address,
        publickey: Public,

    }

    impl Validator {
        fn new(keypair: &KeyPair) -> Validator {
            let publickey = keypair.public();
            let address = keypair.address();
            Validator {
                address,
                publickey: *publickey,
            }
        }
    }

    #[test]
    fn error() {
        writeln!(io::stdout(), "{:?}", Error::Symm).unwrap();
    }


    #[test]
    fn sign() {
        (0..100).for_each(|i| {
            let keypair = Random::generate_keypair();
            let val = Validator::new(&keypair);
            let block = Block::new(i as u64, vec![val]);
            let hash = block.hash();
            writeln!(io::stdout(), "{}: {}", i, ::common::to_hex(hash.as_ref())).unwrap();
            let secp_hash = secp256k1::Message::from_slice(hash.as_ref()).unwrap();
            let signature = sign::sign(&secp_hash, keypair.secret());

            // verify signature
            let ok = sign::verify(keypair.public(), &signature, &hash);
            assert!(ok);
        })
    }
}