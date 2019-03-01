mod print;
mod simple;
mod array_vector;
mod struct_enum;

fn main() {
    
    println!("{}",greet() );
    
    print::go();
    
    
    array_vector::run();

    introduce_yourself("Adam", "adam@test.me");
    let val = multiply(23,5);

    println!("Multiplication {}", val);

    struct_enum::run_struct();
    struct_enum::run_enum();

    //assersion failed as last one
    simple::go();
}
    
fn greet() -> String {
    "Hello, world!".to_string() //return - no additional action but also no action
}

#[test] // test attribute indicates, this is a test function
fn test_greet() {
    assert_eq!("Hello, world!", greet())
}


fn introduce_yourself(name: &str, email: &str){
    println!("My name is {} and my email is {}",name,email );
}

fn multiply(x:i32,y:i32) -> i32{
    x*y
}