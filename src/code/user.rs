use crate::code::model::CodeObject;

pub fn get_user_code_object(code: &str) -> CodeObject {
    match code {
        "CODE_AC_201" => CodeObject::new(
            "CODE_AC_201".to_string(),
            "Account created successfully".to_string(),
            201,
        ),
        "CODE_UNF_400" => CodeObject::new(
            "CODE_UNF_400".to_string(),
            "User not found".to_string(),
            400,
        ),
        "CODE_IP_400" => CodeObject::new(
            "CODE_IP_400".to_string(),
            "Invalid password".to_string(),
            400,
        ),
        "CODE_UAE_400" => CodeObject::new(
            "CODE_UAE_400".to_string(),
            "User already exist".to_string(),
            400,
        ),
        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}