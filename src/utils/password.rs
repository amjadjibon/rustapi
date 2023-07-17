use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::Rng;
use tokio::task;
use anyhow::{anyhow, Context};

use crate::error::password::PasswordError;


pub fn hash_password(password: String) -> Result<String, PasswordError> {
    let salt = SaltString::generate(rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt);
    match password_hash {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(PasswordError::SomethingWentWrong(e.to_string())),
    }
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).unwrap();
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub async fn hash(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("failed to hash password"))?
            .to_string())
    })
        .await
        .context("panic in hash()")?
}

pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow!(e).context("password hash invalid"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("failed to verify password")),
        }
    })
        .await
        .context("panic in verify()")?
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_hash_password() {
//         let password = "password";
//         let hashed_password = hash_password(password);
//         assert_ne!(password, hashed_password);
//     }
//
//     #[test]
//     fn test_verify_password() {
//         let password = "password";
//         let hashed_password = hash_password(password);
//         assert!(verify_password(&hashed_password, password));
//     }
//
//     #[tokio::test]
//     async fn test_hash() {
//         let password = "password";
//         let hashed_password = hash(password.to_string()).await.unwrap();
//         assert_ne!(password, hashed_password);
//     }
//
//     #[tokio::test]
//     async fn test_verify() {
//         let password = "password";
//         let hashed_password = hash(password.to_string()).await.unwrap();
//         assert!(verify(password.to_string(), hashed_password).await.unwrap());
//     }
// }