use dotenv;
use lazy_static::lazy_static;

pub struct Config {
    pub host: String,
    pub port: String,
    pub db_dsn: String,
    pub jwt_secret: String,
    pub jwt_access_exp: i64,
    pub jwt_refresh_exp: i64,
    pub jwt_aud: String,
    pub jwt_iss: String,
    pub jwt_sub: String,
}

lazy_static!(
    pub static ref CONFIG: Config = Config::init();
);

impl Config {
    pub fn init() -> Config {
        Config{
            host: get("HOST"),
            port: get("PORT"),
            db_dsn: get("DB_DSN"),
            jwt_secret: get("JWT_SECRET"),
            jwt_access_exp: get("JWT_ACCESS_EXP").parse::<i64>().unwrap(),
            jwt_refresh_exp: get("JWT_REFRESH_EXP").parse::<i64>().unwrap(),
            jwt_aud: get("JWT_AUD"),
            jwt_iss: get("JWT_ISS"),
            jwt_sub: get("JWT_SUB"),
        }
    }
}

pub fn init() {
    dotenv::dotenv().ok().expect("failed to load .env file");
}

pub fn get(param: &str) -> String {
    std::env::var(param).expect(&format!("failed to get: {param}"))
}
