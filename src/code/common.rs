use crate::code::model::CodeObject;

pub fn get_common_code_object(code: &str) -> CodeObject {
    match code {
        "CODE_VE_400" => CodeObject::new(
            "CODE_VE_400".to_string(),
            "Validation error".to_string(),
            400,
        ),
        "CODE_JR_400" => CodeObject::new(
            "CODE_JR_400".to_string(),
            "JSON Rejection error".to_string(),
            400,
        ),
        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}