use std::io::Write;

fn main() {
    let number: i32 = 50;
    let mut buffer = Vec::with_capacity(10);
    buffer.write_all(&number.to_le_bytes()).unwrap();
}
