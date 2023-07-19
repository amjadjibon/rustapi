use crate::model::code::CodeObject;

pub const CODE_IT_400: &str = "CODE_IT_400";
pub const CODE_TE_400: &str = "CODE_TE_400";
pub const CODE_TM_400: &str = "CODE_TM_400";
pub const CODE_TCF_400: &str = "CODE_TCF_400";
pub const CODE_TDE_400: &str = "CODE_TDE_400";
pub const CODE_ITT_400: &str = "CODE_ITT_400";

pub fn get_code_object(code: &str) -> CodeObject {
    match code {
        CODE_IT_400 => CodeObject::new(
            code.to_string(),
            "Invalid token".to_string(),
            400,
        ),

        CODE_TE_400 => CodeObject::new(
            code.to_string(),
            "Token expired".to_string(),
            400,
        ),

        CODE_TM_400 => CodeObject::new(
            code.to_string(),
            "Token missing".to_string(),
            400,
        ),

        CODE_TDE_400 => CodeObject::new(
            code.to_string(),
            "Token decode error".to_string(),
            400,
        ),

        CODE_TCF_400 => CodeObject::new(
            code.to_string(),
            "Token creation failed".to_string(),
            400,
        ),

        CODE_ITT_400 => CodeObject::new(
            code.to_string(),
            "Invalid token type".to_string(),
            400,
        ),

        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}