// extern crate chrono;
// use chrono::{Utc};

pub fn go(){
    
    let name = "John";
    let age = 32;//variable are immutable
    //in case to be mutable 
    let mut old = true;

    println!("My name is {} and I'm {}, old? {}",name, age, old );
    old = false;
    println!("My name is {} and I'm {}, old? {}",name, age, old );

    //const exists but must be define with type and by convetion name UPPERCASE
    const SALARY: i16 = 4300;
    println!("My salary {} USD", SALARY);

    //tupele
    //IMPORTANT - variable should by snake case
    let (new_name,new_age) = ("Anakin", 41);
    println!("My new name {} and I'm {}", new_name, new_age );
    inner();

}

fn inner(){

    // let now = Utc::now();
    // println!("Month is {}",now.month() );

    let heart_eyed_cat = '😻';
    let unicode_char = '\u{1F923}';
    println!("My nice character type is {}  with {}",heart_eyed_cat, unicode_char);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    

    println!("There is {} right now",months[1] );

    string_operations();

}

fn string_operations(){

    let name = "Alex";

    //String is mutable!- not primitive
    let surname = String::from("Pit");

    println!("Hi my name {} and surname {}", name, surname);
    println!("Hi my name len {} and surname len {}", name.len(), surname.len());

}