use crate::models::user_model::UserRawPhysicalToken;
use std::env;

pub fn serialize_physical_token(token_str: String) -> Result<UserRawPhysicalToken, ()> {
    let decoded_token = branca::decode(token_str.as_str(), env::var("BRANCA_KEY").unwrap().as_ref(), 0);

    return if let Ok(token) = decoded_token {
        Ok(serde_json::from_str::<UserRawPhysicalToken>(&String::from_utf8(token).unwrap()).unwrap())
    } else {
        Err(())
    }
}