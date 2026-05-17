use smart_money::config::load_config;

fn main() {
    let config = load_config();
    println!("{:?}",config);
}