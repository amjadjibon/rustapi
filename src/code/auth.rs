use crate::model::code::CodeObject;

pub const CODE_UAS_200: &str = "CODE_UAS_200";
pub const CODE_UTRS_200: &str = "CODE_UTRS_200";

pub fn get_code_object(code: &str) -> CodeObject {
    match code {
        CODE_UAS_200 => CodeObject::new(
            code.to_string(),
            "User authenticated successfully".to_string(),
            200,
        ),

        CODE_UTRS_200 => CodeObject::new(
            code.to_string(),
            "User token refreshed successfully".to_string(),
            200,
        ),

        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}