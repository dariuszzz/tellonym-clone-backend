use chrono::Duration;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct JWTUtil;

impl JWTUtil {

    
    #[must_use]
    pub fn sign_access_jwt(username: &str) -> String {
        let key: Hmac<Sha256> = Hmac::new_from_slice(dotenv!("ACCESS_SECRET").as_bytes()).unwrap();

        let mut claims = BTreeMap::new();
        claims.insert("sub", username.to_string());
        claims.insert("iad", chrono::offset::Utc::now().to_rfc3339());
        
        claims.sign_with_key(&key).unwrap()
    }
        
    
    #[must_use]
    pub fn verify_access_jwt(jwt: &str) -> Option<String> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(dotenv!("ACCESS_SECRET").as_bytes()).unwrap();
        
        let claims: BTreeMap<String, String> = jwt.verify_with_key(&key).unwrap_or_else(|_| None)?;
        
        let username = claims.get("sub")?.clone();
        let iad = chrono::DateTime::parse_from_rfc3339(claims.get("iad")?).ok()?;
        
        match chrono::offset::Utc::now().signed_duration_since(iad) >= Duration::minutes(15) {
            true  => None,
            false => Some(username)
        }
    }
    
    #[must_use]
    pub fn sign_refresh_jwt(username: &str) -> String {
        let key: Hmac<Sha256> = Hmac::new_from_slice(dotenv!("REFRESH_SECRET").as_bytes()).unwrap();

        let mut claims = BTreeMap::new();
        claims.insert("sub", username.to_string());
        claims.insert("iad", chrono::offset::Utc::now().to_rfc3339());
        
        claims.sign_with_key(&key).unwrap()
    }

    #[must_use]
    pub fn verify_refresh_jwt(jwt: &str) -> Option<String> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(dotenv!("REFRESH_SECRET").as_bytes()).unwrap();
        
        let claims: BTreeMap<String, String> = jwt.verify_with_key(&key).unwrap_or_else(|_| None)?;
        
        let username = claims.get("sub")?.clone();
        let iad = chrono::DateTime::parse_from_rfc3339(claims.get("iad")?).ok()?;
        
        match chrono::offset::Utc::now().signed_duration_since(iad) >= Duration::days(1) {
            true  => None,
            false => Some(username)
        }
    }
}