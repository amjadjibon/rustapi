mod conf;

fn main() {
    conf::env::init();
    println!("Hello, world!");
    println!("DB_HOST: {}", conf::env::get("DB_HOST"));
}
