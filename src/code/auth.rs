use crate::code::model::CodeObject;

pub fn get_common_code_object(code: &str) -> CodeObject {
    match code {
        "CODE_UAS_200" => CodeObject::new(
            "CODE_UAS_200".to_string(),
            "User authentication success".to_string(),
            400,
        ),
        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}