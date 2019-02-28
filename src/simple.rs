extern crate chrono;
use chrono::prelude::*;


fn assertion_test(){

  let mut my_string = String::with_capacity(10);
  my_string.push('a');
  my_string.push('b');

  // Assertion testing
  assert_eq!(3, my_string.len());
  assert_eq!(10, my_string.capacity());
}

pub fn go() {
    println!("########## Hi {} Simple here ##########", "Alex");
    let name = "John";
    let age = 32; //variable are immutable
                  //in case to be mutable
    let mut old = true;

    println!("My name is {} and I'm {}, old? {}", name, age, old);
    old = false;
    println!("My name is {} and I'm {}, old? {}", name, age, old);

    //const exists but must be define with type and by convetion name UPPERCASE
    const SALARY: i16 = 4300;
    println!("My salary {} USD", SALARY);

    //tupele
    //IMPORTANT - variable should by snake case
    let (new_name, new_age) = ("Anakin", 41);
    println!("My new name {} and I'm {}", new_name, new_age);
    inner();
    data_time();
    loop_test();

    assertion_test();
    println!("########## Bey {} Simple here ##########", "Alex");
}

fn data_time(){

    let now = Local::now();

    println!("Month is {}",now.month() );
    
    println!("Local Time is {:?} and day {:?} ", now, now.weekday());

}

fn inner() {
    // let now = Local::now();
    // let mut current_month = 1;
    // if now.month() < 12{
    //     current_month = now.month();
    // }
    let heart_eyed_cat = '😻';
    let unicode_char = '\u{1F923}';
    println!(
        "My nice character type is {}  with {}",
        heart_eyed_cat, unicode_char
    );

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("There is {} right now", months[1]);

    string_operations();
}

fn string_operations() {
    let name = "Alex";

    //String is mutable!- not primitive
    let surname = String::from("Pit and Mart");

    println!("Hi my name {} and surname {}", name, surname);
    println!(
        "Hi my name len {} and surname len {}",
        name.len(),
        surname.len()
    );

    // Check if empty
    println!("Is Empty: {}", surname.is_empty());

    // Contains
    println!("Contains 'Mart' {}", surname.contains("Mart"));

    // Replace
    println!("Replace: {}", surname.replace("Mart", "Obama"));

    // Loop through string by whitespace
    for word in surname.split_whitespace() {
        println!("{}", word);
    }

}

fn loop_test(){

    //fizbuzz test - for loop with range
    for x in 1..19 {
        if x % 15 == 0 {
            println!("fizzbuzz {}", x);
        } else if x % 3 == 0 {
            println!("fizz {}", x);
        } else if x % 5 == 0 {
            println!("buzz {}", x);
        } 
        // else {
        //     println!("{}", x);
        // }
    }
    let mut count = 1;
    //While Loop (FizzBuzz)
    while count <= 23 {
        if count % 15 == 0 {
            println!("while fizzbuzz {}", count);
        } else if count % 3 == 0 {
            println!("while fizz {}", count);
        } else if count % 5 == 0 {
            println!("while buzz {}", count);
        } 
        // else {
        //     println!("while :( {}", count);
        // }
        // Inc
        count += 1;
    }
}
