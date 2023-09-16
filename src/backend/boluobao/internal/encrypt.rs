use crate::share::Timestamp;

use super::consts;
use crypto::{digest::Digest, md5::Md5};
use uuid::Uuid;

pub fn get_sfsecurity(app_version: &str, device_token: &str) -> String {
    assert!(consts::APPKEYS.contains_key(app_version));

    let nonce = Uuid::new_v4().to_string().to_uppercase();
    let timestamp = Timestamp::now().count();
    let device_token = device_token.to_uppercase();
    let appkey = consts::APPKEYS
        .get(app_version)
        .expect("unsupported app version for boluobao backend");

    let source = format!("{nonce}{timestamp}{device_token}{appkey}");
    let mut digest = Md5::new();
    digest.input_str(&source);

    let sign = digest.result_str().to_uppercase();

    format!("nonce={nonce}&timestamp={timestamp}&devicetoken={device_token}&sign={sign}")
}
