use std::io::Read;

fn main() {
    let mut buffer = [0; 10];
    let result = std::io::stdin().read_exact(&mut buffer);
    if result.is_ok() {
        println!("{}", std::str::from_utf8(&buffer).unwrap());
    } else {
        println!("Error!");
    }
}
