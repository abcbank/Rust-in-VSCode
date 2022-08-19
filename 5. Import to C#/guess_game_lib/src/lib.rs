extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub enum CmpResult{
    Equal,
    Small,
    Big
}
#[no_mangle] // 진입점 지정
pub fn create_password(min:i32, max:i32) -> i32{
    return rand::thread_rng().gen_range(min..max);
}
#[no_mangle] // 진입점 지정
pub fn ask_number() -> i32{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    match guess.trim().parse(){
         Ok(num) => return num,
         Err(_) => return i32::min_value()
    };
}
#[no_mangle] // 진입점 지정
pub fn compare_number(pw:i32, to_cmp:i32) -> CmpResult{
    match to_cmp.cmp(&pw){
        Ordering::Less      => return CmpResult::Small,
        Ordering::Greater   => return CmpResult::Big,
        Ordering::Equal     => return CmpResult::Equal
    }
}