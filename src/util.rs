use sha1::{Digest, Sha1};

const GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

#[derive(Debug)]
pub struct WebSocketKey {
    pub key: String,
    pub target: String,
}

pub fn calc_accept_hash(key: &WebSocketKey) -> String {
    let mut sh = Sha1::default();
    sh.update(format!("{}{}", key.key, GUID).as_bytes());
    let output = sh.finalize();
    base64::encode(&output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let hash = calc_accept_hash(&WebSocketKey("dGhlIHNhbXBsZSBub25jZQ==".to_owned()));
        assert_eq!(hash, "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }
}
