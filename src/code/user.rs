use crate::model::code::CodeObject;

pub const CODE_AC_201: &str = "CODE_AC_201";
pub const CODE_UNF_400: &str = "CODE_UNF_400";
pub const CODE_IP_400: &str = "CODE_IP_400";
pub const CODE_UAE_400: &str = "CODE_UAE_400";
pub const CODE_UCF_400: &str = "CODE_UCF_400";


pub fn get_code_object(code: &str) -> CodeObject {
    match code {
        CODE_AC_201 => CodeObject::new(
            code.to_string(),
            "Account created successfully".to_string(),
            201,
        ),

        CODE_UNF_400 => CodeObject::new(
            code.to_string(),
            "User not found".to_string(),
            400,
        ),

        CODE_IP_400 => CodeObject::new(
            code.to_string(),
            "Invalid password".to_string(),
            400,
        ),

        CODE_UAE_400 => CodeObject::new(
            code.to_string(),
            "User already exist".to_string(),
            400,
        ),

        CODE_UCF_400 => CodeObject::new(
            code.to_string(),
            "User creation failed".to_string(),
            400,
        ),

        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}