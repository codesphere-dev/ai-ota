//! End-to-end tests for signing, verification and key encoding.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use ota_crypto::{
    decode_private_key, decode_public_key, encode_private_key, encode_public_key, hex_decode,
    hex_encode, Error, Signature, SigningKey, VerifyingKey, PUBLIC_KEY_LEN, SEED_LEN,
    SIGNATURE_LEN,
};

fn seed(byte: u8) -> [u8; SEED_LEN] {
    [byte; SEED_LEN]
}

#[test]
fn signs_and_verifies_a_message() {
    let key = SigningKey::from_seed(&seed(7));
    let message = b"firmware image v1";

    let signature = key.sign(message);
    assert_eq!(signature.to_bytes().len(), SIGNATURE_LEN);

    let public = key.verifying_key();
    assert!(public.verify(message, &signature).is_ok());
}

#[test]
fn rejects_a_different_message() {
    let key = SigningKey::from_seed(&seed(7));
    let signature = key.sign(b"original");

    assert_eq!(
        key.verifying_key().verify(b"tampered", &signature),
        Err(Error::Verification)
    );
}

#[test]
fn rejects_a_different_key() {
    let signer = SigningKey::from_seed(&seed(7));
    let other = SigningKey::from_seed(&seed(8)).verifying_key();
    let signature = signer.sign(b"payload");

    assert_eq!(
        other.verify(b"payload", &signature),
        Err(Error::Verification)
    );
}

#[test]
fn rejects_a_tampered_signature() {
    let key = SigningKey::from_seed(&seed(7));
    let message = b"payload";

    let mut bytes = key.sign(message).to_bytes();
    bytes[0] ^= 0x01;
    let tampered = Signature::from_bytes(&bytes);

    assert_eq!(
        key.verifying_key().verify(message, &tampered),
        Err(Error::Verification)
    );
}

#[test]
fn verifies_empty_message() {
    let key = SigningKey::from_seed(&seed(1));
    let signature = key.sign(b"");
    assert!(key.verifying_key().verify(b"", &signature).is_ok());
}

#[test]
fn seed_round_trips_through_signing_key() {
    let original = seed(42);
    let key = SigningKey::from_seed(&original);
    assert_eq!(key.to_bytes(), original);
}

#[test]
fn public_key_round_trips_through_bytes() {
    let key = SigningKey::from_seed(&seed(3));
    let public = key.verifying_key();
    let restored = VerifyingKey::from_bytes(&public.to_bytes()).expect("valid point");
    assert_eq!(restored, public);
}

#[test]
fn rejects_wrong_key_lengths() {
    assert!(matches!(
        SigningKey::from_bytes(&[0; 31]),
        Err(Error::InvalidKeyLength {
            expected: SEED_LEN,
            found: 31,
        })
    ));
    assert_eq!(
        VerifyingKey::from_bytes(&[0; PUBLIC_KEY_LEN + 1]),
        Err(Error::InvalidKeyLength {
            expected: PUBLIC_KEY_LEN,
            found: PUBLIC_KEY_LEN + 1,
        })
    );
}

#[test]
fn rejects_wrong_signature_lengths() {
    assert_eq!(
        Signature::from_slice(&[0; SIGNATURE_LEN - 1]),
        Err(Error::InvalidSignatureLength {
            expected: SIGNATURE_LEN,
            found: SIGNATURE_LEN - 1,
        })
    );
    assert_eq!(
        Signature::from_slice(&[0; SIGNATURE_LEN + 1]),
        Err(Error::InvalidSignatureLength {
            expected: SIGNATURE_LEN,
            found: SIGNATURE_LEN + 1,
        })
    );
}

#[test]
fn private_key_file_round_trips() {
    let key = SigningKey::from_seed(&seed(9));
    let encoded = encode_private_key(&key);

    assert!(encoded.contains("BEGIN AI-OTA ED25519 PRIVATE KEY"));
    assert!(encoded.contains("END AI-OTA ED25519 PRIVATE KEY"));

    let decoded = decode_private_key(&encoded).expect("decode private key");
    assert_eq!(decoded.to_bytes(), key.to_bytes());
}

#[test]
fn public_key_file_round_trips() {
    let key = SigningKey::from_seed(&seed(9)).verifying_key();
    let encoded = encode_public_key(&key);

    assert!(encoded.contains("BEGIN AI-OTA ED25519 PUBLIC KEY"));
    assert!(encoded.contains("END AI-OTA ED25519 PUBLIC KEY"));

    let decoded = decode_public_key(&encoded).expect("decode public key");
    assert_eq!(decoded, key);
}

#[test]
fn key_files_are_lowercase_hex() {
    let key = SigningKey::from_seed(&seed(0xab));
    let encoded = encode_private_key(&key);
    assert!(
        encoded.contains("ababab"),
        "expected lowercase hex: {encoded}"
    );
}

#[test]
fn rejects_malformed_key_file_envelope() {
    assert!(matches!(
        decode_private_key("not a key file"),
        Err(Error::MalformedKeyFile)
    ));

    let missing_footer = "-----BEGIN AI-OTA ED25519 PRIVATE KEY-----\n00\n";
    assert!(matches!(
        decode_private_key(missing_footer),
        Err(Error::MalformedKeyFile)
    ));

    let trailing = "-----BEGIN AI-OTA ED25519 PRIVATE KEY-----\n00\n\
                    -----END AI-OTA ED25519 PRIVATE KEY-----\nextra\n";
    assert!(matches!(
        decode_private_key(trailing),
        Err(Error::MalformedKeyFile)
    ));
}

#[test]
fn rejects_public_key_file_read_as_private() {
    let public = encode_public_key(&SigningKey::from_seed(&seed(1)).verifying_key());
    assert!(matches!(
        decode_private_key(&public),
        Err(Error::MalformedKeyFile)
    ));
}

#[test]
fn hex_round_trips() {
    let bytes = [0x00, 0x01, 0x7f, 0x80, 0xfe, 0xff];
    let encoded = hex_encode(&bytes);
    assert_eq!(encoded, "00017f80feff");
    assert_eq!(hex_decode(&encoded).expect("decode"), bytes);
}

#[test]
fn hex_decode_accepts_uppercase() {
    assert_eq!(
        hex_decode("DEADBEEF").expect("decode"),
        [0xde, 0xad, 0xbe, 0xef]
    );
}

#[test]
fn hex_decode_rejects_odd_length() {
    assert_eq!(hex_decode("abc"), Err(Error::InvalidHexLength(3)));
}

#[test]
fn hex_decode_rejects_non_hex_characters() {
    assert_eq!(hex_decode("zz"), Err(Error::InvalidHexByte(b'z')));
    assert_eq!(hex_decode("0g"), Err(Error::InvalidHexByte(b'g')));
}
