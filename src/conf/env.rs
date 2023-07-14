use dotenv;

pub fn init() {
    dotenv::dotenv().ok().expect("failed to load .env file");
}

pub fn get(param: &str) -> String {
    std::env::var(param).expect(&format!("failed to get: {param}"))
}
