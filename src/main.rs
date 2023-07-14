mod conf;

use conf::env;

fn main() {
    env::init();
    println!("DB_HOST: {}", env::get("DB_HOST"));
}
