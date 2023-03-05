fn main() {
    let optional_var1: Option<i32> = Some(10);
    let optional_var2: Option<i32> = None;

    if let Some(optional_var1) = optional_var1 {
        println!("optional_var1 has value of {}", optional_var1);
    } else {
        println!("optional_var1 has no value!");
    }

    if let Some(optional_var2) = optional_var2 {
        println!("optional_var2 has value of {}", optional_var2);
    } else {
        println!("optional_var2 has no value!");
    }
}
