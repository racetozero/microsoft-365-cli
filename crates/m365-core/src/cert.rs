//! Certificate login: reads a PEM or PFX certificate (stored base64-encoded
//! in the connection, as upstream does), computes the SHA-1 thumbprint and
//! signs the client assertion JWT.

use crate::auth::{CERT_BASE64, CERT_BINARY, CERT_UNKNOWN, Connection};
use crate::error::{Error, Result};
use base64::Engine;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::signature::{SignatureEncoding, Signer};
use sha1::Digest;

fn b64() -> base64::engine::GeneralPurpose {
    base64::engine::general_purpose::STANDARD
}

fn pem_blocks(text: &str) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut lines = text.lines();
    while let Some(l) = lines.next() {
        let l = l.trim();
        if let Some(label) = l
            .strip_prefix("-----BEGIN ")
            .and_then(|x| x.strip_suffix("-----"))
        {
            let mut body = String::new();
            for l in lines.by_ref() {
                let l = l.trim();
                if l.starts_with("-----END ") {
                    break;
                }
                if !l.contains(':') {
                    body.push_str(l);
                }
            }
            if let Ok(der) = b64().decode(body) {
                out.push((label.to_string(), der));
            }
        }
    }
    out
}

/// `Buffer.from(s, 'base64')`: skips characters outside the alphabet and
/// stops at padding, like Node.
pub fn lenient_base64(s: &str) -> Vec<u8> {
    let clean: String = s
        .chars()
        .map(|c| match c {
            '-' => '+',
            '_' => '/',
            c => c,
        })
        .take_while(|c| *c != '=')
        .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '/')
        .collect();
    let usable = clean.len() - clean.len() % 4
        + if clean.len() % 4 > 1 {
            clean.len() % 4
        } else {
            0
        };
    base64::engine::general_purpose::STANDARD_NO_PAD
        .decode(&clean[..usable.min(clean.len())])
        .unwrap_or_default()
}

pub fn thumbprint(der: &[u8]) -> String {
    let h = sha1::Sha1::digest(der);
    h.iter().map(|b| format!("{b:02x}")).collect()
}

/// Returns (PKCS#8 PEM private key, thumbprint, certificate type).
pub fn load(conn: &Connection) -> Result<(String, String, u8)> {
    let raw = lenient_base64(conn.certificate.as_deref().unwrap_or(""));
    let mut ty = conn.certificate_type;
    if ty == CERT_UNKNOWN || ty == CERT_BASE64 {
        let text = String::from_utf8_lossy(&raw);
        let blocks = pem_blocks(&text);
        let key = blocks.iter().find(|(l, _)| l.ends_with("PRIVATE KEY"));
        if let Some((label, der)) = key {
            let key = match label.as_str() {
                "RSA PRIVATE KEY" => {
                    use rsa::pkcs1::DecodeRsaPrivateKey;
                    rsa::RsaPrivateKey::from_pkcs1_der(der)
                        .map_err(|e| Error::msg(e.to_string()))?
                }
                "ENCRYPTED PRIVATE KEY" => rsa::RsaPrivateKey::from_pkcs8_encrypted_der(
                    der,
                    conn.password.as_deref().unwrap_or("").as_bytes(),
                )
                .map_err(|e| Error::msg(e.to_string()))?,
                _ => rsa::RsaPrivateKey::from_pkcs8_der(der)
                    .map_err(|e| Error::msg(e.to_string()))?,
            };
            let pem = key
                .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                .map_err(|e| Error::msg(e.to_string()))?
                .to_string();
            let tp = match &conn.thumbprint {
                Some(t) => t.clone(),
                None => {
                    let cert = blocks
                        .iter()
                        .find(|(l, _)| l == "CERTIFICATE")
                        .ok_or_else(|| Error::msg("Certificate not found"))?;
                    thumbprint(&cert.1)
                }
            };
            return Ok((pem, tp, ty));
        }
        ty = CERT_BINARY;
    }
    // PFX
    let ks = p12_keystore::KeyStore::from_pkcs12(&raw, conn.password.as_deref().unwrap_or(""))
        .map_err(|e| Error::msg(format!("{e:?}")))?;
    for (_, entry) in ks.entries() {
        if let p12_keystore::KeyStoreEntry::PrivateKeyChain(chain) = entry {
            let key = rsa::RsaPrivateKey::from_pkcs8_der(chain.key())
                .map_err(|e| Error::msg(e.to_string()))?;
            let pem = key
                .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                .map_err(|e| Error::msg(e.to_string()))?
                .to_string();
            let tp = match &conn.thumbprint {
                Some(t) => t.clone(),
                None => chain
                    .chain()
                    .first()
                    .map(|c| thumbprint(c.as_der()))
                    .unwrap_or_default(),
            };
            return Ok((pem, tp, ty));
        }
    }
    Err(Error::msg("No private key found in the certificate"))
}

fn b64url(b: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b)
}

/// The signed JWT MSAL sends as `client_assertion`.
pub fn client_assertion(
    key_pem: &str,
    thumbprint_hex: &str,
    client_id: &str,
    audience: &str,
) -> Result<String> {
    let tp: Vec<u8> = (0..thumbprint_hex.len() / 2)
        .filter_map(|i| u8::from_str_radix(&thumbprint_hex[i * 2..i * 2 + 2], 16).ok())
        .collect();
    let header = serde_json::json!({ "alg": "RS256", "typ": "JWT", "x5t": b64url(&tp) });
    let now = chrono::Utc::now().timestamp();
    let claims = serde_json::json!({
        "aud": audience, "exp": now + 600, "iss": client_id, "jti": uuid::Uuid::new_v4().to_string(), "nbf": now, "sub": client_id, "iat": now
    });
    let signing_input = format!(
        "{}.{}",
        b64url(header.to_string().as_bytes()),
        b64url(claims.to_string().as_bytes())
    );
    let key = rsa::RsaPrivateKey::from_pkcs8_pem(key_pem).map_err(|e| Error::msg(e.to_string()))?;
    let signer = SigningKey::<sha2::Sha256>::new(key);
    let sig = signer.sign(signing_input.as_bytes());
    Ok(format!("{signing_input}.{}", b64url(&sig.to_bytes())))
}
