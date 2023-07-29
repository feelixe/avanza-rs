use otpauth::TOTP;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn generate_totp(secret: &str) -> u32 {
    let auth = TOTP::from_base32(secret).unwrap();
    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let code = auth.generate(30, timestamp1);
    return code;
}
