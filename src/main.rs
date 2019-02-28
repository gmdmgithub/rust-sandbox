mod print;
mod simple;
mod array_vector;

fn main() {
    
    println!("{}",greet() );
    
    print::go();
    
    
    array_vector::run();


    //assersion failed as last one
    simple::go();
}
    
fn greet() -> String {
    "Hello, world!".to_string()
}

#[test] // test attribute indicates, this is a test function
fn test_greet() {
    assert_eq!("Hello, world!", greet())
}