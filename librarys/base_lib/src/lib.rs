use rand::Rng;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn get_random_number(min:i32, max:i32) -> i32{
    return rand::thread_rng().gen_range(min..max);
}

pub fn fibonacci(x: i32) -> i32 {
    if x == 0 { return 0; }
    else if x == 1 { return 1; }
    else { return fibonacci(x-1) + fibonacci(x-2); }
}