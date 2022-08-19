extern crate rand;

use std::io;
use rand::Rng;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn create_password(min:i32, max:i32) -> i32{
    return rand::thread_rng().gen_range(min..max);
}

pub fn ask_number() -> i32{
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    match guess.trim().parse(){
         Ok(num) => return num,
         Err(_) => return i32::min_value()
    };
}