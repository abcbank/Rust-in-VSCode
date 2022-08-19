use time::Duration;
use time::macros::{datetime};
use build_crate;
fn main() {
    time_test();
    println!("build_crate::sub: {}", build_crate::sub(10, 7));
    println!("build_crate::add: {}", build_crate::add(10, 7));
    println!("build_crate::mul: {}", build_crate::mul(10, 7));
    println!("build_crate::div: {}", build_crate::div(10, 7));
}

fn time_test(){
    let a = datetime!(2022-01-01 16:42:34);
    let b = datetime!(2022-01-01 12:35:12);

    let duration:Duration = b - a;
    println!("{}", b - a); 
}
