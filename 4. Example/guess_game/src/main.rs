extern crate guess_game_lib;

use std::cmp::Ordering;

fn main() {
    let pw = guess_game_lib::create_password(0,100);
    println!("PW: {}", pw);

    loop{
        let guess = guess_game_lib::ask_number();
        if guess == i32::min_value() {
            println!("Please Type Number");
            continue;
        }
        println!("Guess Number: {}", guess);

        match guess.cmp(&pw){
            Ordering::Less      => is_smaller(),
            Ordering::Greater   => is_bigger(),
            Ordering::Equal     => { is_same(); break; }
        }
    } 
}
fn is_bigger(){
    println!("Too big!");
}
fn is_smaller(){
    println!("Too small!");
}
fn is_same(){
    println!("You win!");
}
