use crate::model::code::CodeObject;

pub const CODE_RS_200: &str = "CODE_RS_200";
pub const CODE_RF_400: &str = "CODE_RF_400";
pub const CODE_VE_400: &str = "CODE_VE_400";
pub const CODE_JR_400: &str = "CODE_JR_400";
pub const CODE_UCV_409: &str = "CODE_UCV_409";


pub fn get_code_object(code: &str) -> CodeObject {
    match code {
        CODE_RS_200 => CodeObject::new(
            code.to_string(),
            "Request success".to_string(),
            200,
        ),

        CODE_RF_400 => CodeObject::new(
            code.to_string(),
            "Request failed".to_string(),
            400,
        ),

        CODE_VE_400 => CodeObject::new(
            code.to_string(),
            "Validation error".to_string(),
            400,
        ),

        CODE_JR_400 => CodeObject::new(
            code.to_string(),
            "Json error".to_string(),
            400,
        ),

        CODE_UCV_409 => CodeObject::new(
            code.to_string(),
            "User already exist".to_string(),
            409,
        ),

        _ => CodeObject::new(
            "CODE_UE_400".to_string(),
            "Unknown error".to_string(),
            400,
        ),
    }
}