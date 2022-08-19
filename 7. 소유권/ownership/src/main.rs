fn main() {
    // ownership_heap1();
    // ownership_heap2();
    // ownership_heap3();
    // ownership_heap4();
    // ownership_heap5();
    // ownership_stack1();
    // ownership_stack2();
    // ownership_stack3();
    // slice();
    reference();
}
fn reference(){
    let mut a = String::from("ABCBank");
    // 참조자를 통해 값 전달
    println!("{}", fuction_reference(&a));
    // 참조자를 통해 값을 전달했으므로 이후에도 동일하게 a 접속 가능
    println!("{}", a);
}
fn slice(){
    // String 타입
    let mut a = String::from("ABCBank");
    println!("{}", a);
    // &str 타입 -> 소유권이 없음
    let mut b = &a[0..3];
    //재초기화할 경우 &str의 소유권이 동시에 사라지므로 컴파일 에러 발생
    //a = String::from("DEFBank");
    println!("{}", a);
    println!("{}", b);
}
fn ownership_stack1(){
    let a = [0,1,2,3,4,5];
    println!("{}", a[0]);
    let b = a;
    println!("{}", a[0]);
    println!("{}", b[0]);
    // 소유권이 a에서 b로 옮겨졌기 때문에 아래의 명령어를 수행시 에러 발생
    //println!("{}", a);
}
fn ownership_stack2(){
    let a = 5;
    // 소유권이 함수의 파라미터로 이동
    let b = fuction_array(a);
    // a는 함수를 수행 후 소유권이 존재하지 않으므로 직접적인 접속 불가능
    println!("{}", a);
    println!("{}", b);
}
fn ownership_stack3(){
    let a = 5;
    // a의 소유권이 함수 파라미터로 이동
    let a = fuction_array(a);
    // a의 소유권은 함수 수행 후 사라지나, 함수의 리턴값을 새로 할당받아 소유권을 건네줌.
    println!("{}", a);
}
fn ownership_heap1(){
    let a = String::from("ABCBank");
    println!("{}", a);
    let b = a;
    println!("{}", b);
    // 소유권이 a에서 b로 옮겨졌기 때문에 아래의 명령어를 수행시 에러 발생
    //println!("{}", a);
}
fn ownership_heap2(){
    let a = String::from("ABCBank");
    let b = a.clone();
    println!("{}", a);
    println!("{}", b);
}
fn ownership_heap3(){
    let a = String::from("ABCBank");
    // 소유권이 함수의 파라미터로 이동
    let b = fuction_string(a);
    // a는 함수를 수행 후 소유권이 존재하지 않으므로 직접적인 접속 불가능
    //println!("{}", a);
    println!("{}", b);
}
fn ownership_heap4(){
    let a = String::from("ABCBank");
    // a의 복사본의 소유권이 함수 파라미터로 이동
    let b = fuction_string(a.clone());
    // 함수 수행 후 a의 복사본은 사라지나, a의 소유권은 그대로이므로 접속 가능
    println!("{}", a);
    println!("{}", b);
}
fn ownership_heap5(){
    let a = String::from("ABCBank");
    // a의 소유권이 함수 파라미터로 이동
    let a = fuction_string(a);
    // a의 소유권은 함수 수행 후 사라지나, 함수의 리턴값을 새로 할당받아 소유권을 건네줌.
    println!("{}", a);
}
fn fuction_string(value:String) -> String{
    value.replace("B", "")
    // 함수의 스코프가 종료됨과 동시에 Value의 할당 해제
}
fn fuction_array(value:i32) -> i32{
    value + 1
}
fn fuction_reference(value:&String) -> String{
    value.replace("B", "")
}