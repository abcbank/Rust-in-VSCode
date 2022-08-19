use std::io;
use std::io::Write;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub fn get_string()->String{
    print!("Input: ");
    match io::stdout().flush(){
        Ok(_) => {
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            input
        },
        Err(_) => { return String::new(); }
    }
}

pub fn get_number()->i32{
    print!("Input: ");
    match io::stdout().flush() {
        Ok(_) => {
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            match input.trim().parse(){
                Ok(num) => return num,
                Err(_) => return i32::min_value()
            };
        },
        Err(_) => { return i32::min_value(); }
    }
}
