#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn add(x: i32, y:i32) -> i32{
    x+y
}
pub fn div(x:i32, y:i32) -> i32{
    x/y
}
pub fn sub(x: i32, y:i32) -> i32{
    x-y
}
pub fn mul(x:i32, y:i32) -> i32{
    x*y
}