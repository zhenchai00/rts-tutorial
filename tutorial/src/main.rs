fn main() {
    println!("Hello, world!");

    let _x: i32 = -42;   // signed integer max value is 2147483647
    let _y: i64 = 100;   // unsigned integer max value is 9223372036854775807
    let x: i32 = 2147483647;
    let y: i64 = 9223372036854775807;
    println!("signed integer: {}", x);
    println!("unsigned integer: {}", y);


    // float 
    let pi: f64 = 3.14;
    println!("float: {}", pi);

    // boolean
    let is_true: bool = true;
    println!("boolean: {}", is_true);

    // char
    let letter: char = 'A';
    println!("char: {}", letter);


    // array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("number array: {:?}", numbers);
    // let mix = [1, 2, "apple", true];
    // println!("mix array: {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("fruits array: {:?}", fruits);
    println!("first fruit: {}", fruits[0]);
    println!("second fruit: {}", fruits[1]);
    println!("third fruit: {}", fruits[2]);


    // tuple
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human: {:?}", human);
    let my_mix_tuple = ("AAA".to_string(), 100, true, [1, 2, 3, 4, 5]);
    println!("My mix tuple: {:?}", my_mix_tuple);


    // slice: contiguous sequence of elements in a collection
    // it related to memory allocation and performance
    let number_slices: &[i32] = &[1, 2, 3, 4];
    println!("number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["dog", "cat", "bird"];
    println!("animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"WHAT".to_string(), &"HAHAHA".to_string()];
    println!("book slices: {:?}", book_slices);


    // String vs String Slices (&str)
    let mut stone_cold: String = String::from("hell, ");
    println!("stone cold: {}", stone_cold);
    stone_cold.push_str("yeah!");
    println!("stone cold: {}", stone_cold);

    // &str (string slices) references
    let string: String = String::from("hello, world!");
    let slice: &str = &string[0..5]; // will only show the first 5 characters
    println!("slice value: {}", slice);

    hello_world();
    tell_height(180);
    human_id("Alice", 30, 180.5);

    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("x: {}", _x);
    let y = add(10, 20);
    println!("y: {}", y);

    // bmi 
    let weight = 50.0;
    let height = 1.6;
    let bmi = calculate_bmi(weight, height);
    println!("BMI: {}", bmi);


    // let s1 = String::from("RUST");
    // let s2 = s1;
    // let len = calculate_length(&s1);
    // println!("Length of {} : {}", s1, len);

    // let mut _x = 5;
    // let _r = &mut _x;
    // *_r += 3;
    // *_r -= 2;
    // println!("value of _x : {}", _x);
    // println!("value of _r : {}", _r);

    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 103.33,
    };
    // immutable borrow to check balance
    account.check_balance();
    // mutable borrow to check balance
    account.withdraw(22.3);
    // immutable borrow to check balance
    account.check_balance();
}

fn hello_world() {
    println!("Hello, world!");
}

fn tell_height(height: i32) {
    println!("Height: {}", height);
}

fn human_id(name: &str, age:u32, height: f32) {
    println!("Name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Balance of account {} is {}", self.owner, self.balance);
    }
}