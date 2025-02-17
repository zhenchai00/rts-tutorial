use std::fs::File;

fn main() {
    let f = File::open("some_document.txt");
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        }
        Err(e) => {
            println!("file not found {:?}", e);
        }
    }
    println!("end of main");
}