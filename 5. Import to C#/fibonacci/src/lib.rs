#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
#[no_mangle]
pub extern fn fibonacci(x: i32) -> i32 {
    if x == 0 { return 0; }
    else if x == 1 { return 1; }
    else { return fibonacci(x-1) + fibonacci(x-2); }
}