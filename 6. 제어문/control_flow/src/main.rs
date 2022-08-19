use std::io;
use base_lib;
use io_control;

fn main() {
    //control_if_1();
    //control_if_2();
    //control_loop();
    //control_while_1();
    //control_while_2();
    control_for();
}

fn control_if_1(){
    println!("if 제어문 테스트 함수 1");
    let x = io_control::get_number();
    // 조건식에는 항상 bool 자료형만 들어가야함.
    if x > 0 {
        println!("{}는 양수입니다.", x);
    }
    else if x == 0 {
        println!("{}는 양수도 음수도 아닙니다.", x);
    }
    else if x == i32::min_value() {
        println!("정수가 아닌 숫자가 입력되었습니다.");
    }
    else{
        println!("{}는 음수입니다.", x);
    }
    println!();
}

fn control_if_2(){
    println!("if 제어문 테스트 함수 2");
    let x = io_control::get_number();
    // 단, 아래와 같은 경우 if 문 내부에서 리턴되는 값은 자료형이 동일해야함.
    let y = if x > 0 { x } else { 0 };
    // let y = if x > 0 { 'A' } else { 0 }; << 'A'와 0은 자료형이 서로 달라 불가능
    println!("{}", y);
    println!();
}
fn control_loop(){
    println!("loop 제어문 테스트 함수 2");
    loop{
        println!("loop에서 벗어나기 위해선 숫자 0를 입력해주세요.");
        let x = io_control::get_number();
        if x == 0 {
            break;
        }
    }
    println!();
}
fn control_while_1(){
    println!("while 제어문 테스트 함수 1");
    let mut x = -1;
    while x != 0 {
        println!("while 벗어나기 위해선 숫자 0를 입력해주세요.");
        x = io_control::get_number();
    }
    println!();
}

fn control_while_2(){
    println!("while 제어문 테스트 함수 2");
    println!("차례대로 저장할 숫자를 입력해주세요.");
    let mut arr:[i32; 5] = [0,0,0,0,0];
    let mut index = 0;
    while  index < 5 {
        println!("Current Index: {}", index);
        let value = io_control::get_number();
        if value != i32::min_value() {
            arr[index as usize] = value;
            index += 1;
        }
        else{
            println!("정수를 입력해주세요.");
        }
    }
    for element in arr.iter() {
        println!("Value: {}", element);
    }
    println!();
}

fn control_for(){
    println!("for 제어문 테스트 함수 1");
    println!("배열 내 모든 변수 출력");
    let arr:[i32; 5] = [4,5,1,3,4];
    for element in arr.iter() {
        println!("Array Value: {}", element);
    }
    println!();
}