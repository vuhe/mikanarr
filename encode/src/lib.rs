pub fn sha1_encode(data: impl AsRef<[u8]>) -> String {
    use sha1::{Digest, Sha1};
    hex::encode(Sha1::digest(data))
}

pub fn sha256_encode(data: impl AsRef<[u8]>) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(data))
}

pub fn base64_encode(data: impl AsRef<[u8]>) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}
