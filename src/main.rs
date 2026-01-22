mod json;

fn main() {
    let fifty_three = json::Json::hex_to_dec_32_bit("0035".as_bytes());
    println!("{:?}", fifty_three);
    
    println!("'A' = {}", 'A' as u32);
    println!("'a' = {}", 'a' as u32);
}
