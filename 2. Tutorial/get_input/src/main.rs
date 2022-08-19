use std::io;

fn main() {
    parameter_test();
    println!("Guess the number!");

    println!("Please input your guess.");

    // let mut guess = String::new();
    
    // // & 포인터 개념. 해당 객체에 직접적으로 접속
    // io::stdin().read_line(&mut guess)
    // // read_line: Result(OK/Err) 타입 리턴
    //     .expect("Failed to read line");
    //     // OK: 결과값의 바이트 수 리턴  Err: 프로그램 정지 후 메세지 출력
        
    // println!("You guessed: {}", guess);
}

fn parameter_test(){
    raw_identifier_test();
    immutable_test();
    mutable_test();
    shadowing_test();
    scalar_types();
    complex_types();
    ownership();
}

fn raw_identifier_test(){
    let r#fn = "Raw Identifier";
    println!("Raw Identifier Test: {}", r#fn);
}

fn immutable_test(){
    // let var = 4;
    // var = 2;
    let var;
    var = 4;
    println!("Immutable Test: {}", var);
}

fn mutable_test(){
    let mut var = 2;
    println!("Before Mute: {}", var);
    var = 5;
    println!("After Mute: {}", var);
}

fn shadowing_test(){
    let var = 43;
    println!("Before Shadowing: {}", var);
    let var = "twenty four";
    println!("After Shadowing: {}", var);
}

fn scalar_types(){
    println!("\r\nScalar Types");
    println!("Integer / Unsigned Integer: i8(u8) / i16(u16) / i32(u32) / i64(u64) / i128(u128)\r\n");
    println!("기타 언어들과 동일하게 할당된 바이트 수에 따라 표현 가능한 크기가 다름.");
    let max_value = i8::max_value();
    let min_value = i8::min_value();
    println!("i8 Max Value: {}", max_value);
    println!("i8 Min Value: {}", min_value);
    let max_value = u8::max_value();
    let min_value = u8::min_value();
    println!("u8 Max Value: {}", max_value);
    println!("u8 Min Value: {}", min_value);
    let max_value = i16::max_value();
    let min_value = i16::min_value();
    println!("i16 Max Value: {}", max_value);
    println!("i16 Min Value: {}", min_value);
    let max_value = u16::max_value();
    let min_value = u16::min_value();
    println!("u16 Max Value: {}", max_value);
    println!("u16 Min Value: {}", min_value);
    let max_value = i32::max_value();
    let min_value = i32::min_value();
    println!("i32 Max Value: {}", max_value);
    println!("i32 Min Value: {}", min_value);
    let max_value = u32::max_value();
    let min_value = u32::min_value();
    println!("u32 Max Value: {}", max_value);
    println!("u32 Min Value: {}", min_value);
    let max_value = i64::max_value();
    let min_value = i64::min_value();
    println!("i64 Max Value: {}", max_value);
    println!("i64 Min Value: {}", min_value);
    let max_value = u64::max_value();
    let min_value = u64::min_value();
    println!("u64 Max Value: {}", max_value);
    println!("u64 Min Value: {}", min_value);
    let max_value = i128::max_value();
    let min_value = i128::min_value();
    println!("i128 Max Value: {}", max_value);
    println!("i128 Min Value: {}", min_value);
    let max_value = u128::max_value();
    let min_value = u128::min_value();
    println!("u128 Max Value: {}", max_value);
    println!("u128 Min Value: {}", min_value);
    
    println!("\r\nFloat: f32 / f64\r\n");
    println!("성능 차이는 거의 없으나, 더 상세한 값을 표현 가능한 f64를 자주 사용\r\n");
    println!("Min / Max 표시의 경우, 표현 불가능하므로 제외");

    println!("\r\nBoolean: bool\r\n");
    println!("참 / 거짓에 대응\r\n");
    println!("bool::true: {}", true);
    println!("bool::false: {}", false);

    println!("\r\nChar: char\r\n");
    println!("문자 하나에 대응. 유니코드 기반 문자형");
    let test_value = '맞';
    println!("test_value: {}\r\n", test_value);
}

fn complex_types(){
    println!("\r\nScalar Types");
    println!("Tuple: (T1, T2, T3, ...)\r\n");
    println!("여러 개의 스칼라 타입을 하나의 자료형으로 관리. 인덱스를 통해 각 값을 참조 가능.");
    let three_value_tuple = (500, 1.5, 'ㅁ');
    println!("Value 1: {}", three_value_tuple.0);
    println!("Value 2: {}", three_value_tuple.1);
    println!("Value 3: {}", three_value_tuple.2);
    // 동일하게 mut 태그가 없는 변수는 각 스칼라 타입을 재할당할 수 없음. 
    // three_value_tuple.0 = 115;
    println!("튜플 자료형을 여러 개의 스칼라 타입으로 해체할 수 있음.");
    let (x,y,z) = three_value_tuple;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    println!("\r\nArray: [T1; length]\r\n");
    println!("동일한 스칼라 타입을 하나의 자료형으로 관리. 인덱스를 통해 각 값을 참조 가능.");
    let array = [0,1,2,3,4,5,6,7,8,9];
    let mut counter = 0;
    for element in array.iter(){
        println!("Array {}: {}", counter, element);
        counter += 1;
    }
    println!("\r\n");
    // 동일하게 mut 태그가 없는 변수는 각 스칼라 타입을 재할당 할 수 없음.
    // array[0] = 115;
}

fn ownership(){
    let test_string = String::from("abcBank");
    test_function(test_string);
    // 재호출시 에러 발생 -> 오너쉽이 함수 파라미터를 통해 전해졌고, 함수 종료와 동시에 test_string의
    // 메모리 할당을 해제. 따라서 두 번 이상 접근할 수 없음.
    // 위와 같은 경우, test_string.clone()을 활용하는 것이 바람직함.
    // test_function(test_string);

}
fn test_function(string_value:String){
    println!("Parameter String: {}", string_value);
}