use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,         // Optional. Audience
    pub exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,          // Optional. Issued at (as UTC timestamp)
    pub iss: String,         // Optional. Issuer
    pub nbf: usize,          // Optional. Not Before (as UTC timestamp)
    pub sub: String,         // Optional. Subject (whom token refers to)
    pub token_type: String,  // Optional. Type of the token (JWT) e.g. access or refresh
    pub user_id: i32,        // Optional. User ID
}