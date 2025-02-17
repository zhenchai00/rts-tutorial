fn main() {
    let num = even_checker(12);
    match num {
        Ok(num) => {
            println!("{} is even", num);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn even_checker(num: i32) -> Result<bool, String> {
    if num % 2 == 0 {
        Ok(true)
    } else {
        Err(String::from("Number is not even"))
    }
}