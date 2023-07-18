use crate::code::model::CodeObject;

pub fn get_code_object(code: &str) -> CodeObject {
    match code {
        "CODE_IT_400" => CodeObject::new(
            "CODE_IT_400".to_string(),
            "Invalid token".to_string(),
            400,
        ),
        "CODE_TE_400" => CodeObject::new(
            "CODE_TE_400".to_string(),
            "Token expired".to_string(),
            400,
        ),
        "CODE_TM_400" => CodeObject::new(
            "CODE_TM_400".to_string(),
            "Token missing".to_string(),
            400,
        ),
        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}