use crate::model::{token::{Claims}, user::User};
use chrono;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use crate::conf::env;
use crate::error::token::TokenError;
use crate::model::login::{UserLoginRefreshResponseDto, UserLoginResponseDto};

#[derive(Clone)]
pub struct TokenService {
    secret: String,
    access_token_exp: i64,
    refresh_token_exp: i64,
}

pub trait TokenServiceTrait {
    fn new() -> Self;
    fn login(&self, user: User) -> Result<UserLoginResponseDto, TokenError>;
    fn login_refresh(&self, refresh_token: String) -> Result<UserLoginRefreshResponseDto, TokenError>;
    const ACCESS_TOKEN_EXPIRATION: i64;
    const REFRESH_TOKEN_EXPIRATION: i64;
}

impl TokenService {
    const ACCESS_TOKEN_EXPIRATION: i64 = 60;
    const REFRESH_TOKEN_EXPIRATION: i64 = 900;

    pub fn new() -> Self {
        let access_token_exp = env::get("JWT_ACCESS_EXP")
            .parse()
            .unwrap_or(Self::ACCESS_TOKEN_EXPIRATION);

        let refresh_token_exp = env::get("JWT_REFRESH_EXP")
            .parse()
            .unwrap_or(Self::REFRESH_TOKEN_EXPIRATION);


        return Self {
            secret: env::get("JWT_SECRET"),
            access_token_exp,
            refresh_token_exp,
        };
    }

    pub fn login(&self, user: User) -> Result<UserLoginResponseDto, TokenError> {
        let access_token = self.create_access_token(user.id)?;
        let refresh_token = self.create_refresh_token(user.id)?;

        let response = UserLoginResponseDto {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_exp,
        };

        Ok(response)
    }

    pub fn login_refresh(
        &self,
        refresh_token: String,
    ) -> Result<UserLoginRefreshResponseDto, TokenError> {
        let token_data = self.decode_token(&refresh_token)?;

        if token_data.claims.token_type != "refresh_token" {
            return Err(TokenError::InvalidTokenType);
        }

        let access_token = self.create_access_token(token_data.claims.user_id)?;
        let refresh_token = self.create_refresh_token(token_data.claims.user_id)?;

        let response = UserLoginRefreshResponseDto {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_exp,
        };

        Ok(response)
    }

    pub fn decode_token(&self, token: &str) -> Result<TokenData<Claims>, TokenError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        );

        let token_data = match token_data {
            Ok(token_data) => token_data,
            Err(e) => return Err(TokenError::TokenDecodeError(e.to_string())),
        };

        Ok(token_data)
    }


    fn create_access_token(&self, user_id: i32) -> Result<String, TokenError> {
        let claims = Claims {
            aud: env::get("JWT_AUDIENCE"),
            exp: chrono::Utc::now().timestamp() as usize + self.access_token_exp as usize,
            iat: chrono::Utc::now().timestamp() as usize,
            iss: env::get("JWT_ISSUER"),
            nbf: chrono::Utc::now().timestamp() as usize,
            sub: env::get("JWT_SUBJECT"),
            token_type: "access_token".to_string(),
            user_id,
        };

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        );

        let access_token = match access_token {
            Ok(token) => token,
            Err(e) => return Err(TokenError::TokenCreationError(e.to_string())),
        };

        Ok(access_token)
    }

    fn create_refresh_token(&self, user_id: i32) -> Result<String, TokenError> {
        let claims = Claims {
            aud: env::get("JWT_AUDIENCE"),
            exp: chrono::Utc::now().timestamp() as usize + self.refresh_token_exp as usize,
            iat: chrono::Utc::now().timestamp() as usize,
            iss: env::get("JWT_ISSUER"),
            nbf: chrono::Utc::now().timestamp() as usize,
            sub: env::get("JWT_SUBJECT"),
            token_type: "refresh_token".to_string(),
            user_id,
        };

        let refresh_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        );

        let refresh_token = match refresh_token {
            Ok(token) => token,
            Err(e) => return Err(TokenError::TokenCreationError(e.to_string())),
        };

        Ok(refresh_token)
    }
}
