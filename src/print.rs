pub fn go() {
    println!("########## Hi {} prints here ##########", "Alex");
    println!("My name is {} and I'm {}", "Stive", 27);

    //position is like below
    println!("{0} is from {1} and {0} is {2}", "Mike", "NY", 23);

    //named parameters
    println!("{name} is form {city}", city = "Orlando", name = "John");
    
    //debug traits - with tuple (like in python)
    println!("Check {:?}",(12,true,"Sam") );

    //Traits placeholders
    println!("Binary {:b} Octo {:o} Hex {:x} for number {number}" , 12, 12, 12, number=12 );

    println!("########## Bey {} prints here ##########", "Alex");
}
