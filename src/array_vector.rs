use std::mem;

pub fn run(){
    println!("########## Hi {} Array and vect here ##########", "Alex");
    //mut - mutable, [type,size]
    let mut first_array: [i16; 5] = [2, 1, 8, 5, 12];

    // reassigne is simple
    first_array[1] = 3;

    println!("{:?}", first_array);

    println!("Print some value: {}", first_array[0]);

    println!("Get array length: {}", first_array.len());

    //println!("Array occupies {} bytes", std::mem::size_of_val(&first_array));
    // ! simpler by use std::mem
    println!("Array occupies {} bytes", mem::size_of_val(&first_array));

    // How to slice?? - like other with ... 
    let slice = &first_array[1..3];
    println!("Slice: {:?}", slice);
    let slice_first = &first_array[..2];
    println!("Slice first: {:?}", slice_first);
    let slice_last = &first_array[3..];
    println!("Slice last: {:?}", slice_last);


    // do the same with vect
    run_vec();

    println!("########## Bye {} Array and vect here ##########", "Alex");
}

fn run_vec(){

    println!("###### Hi {} vector ####", "Alex");
    //mut - mutable [type] - no size!!
    let mut first_vector: Vec<i16> = vec![2, 1, 8, 5, 12];

    // reassigne is simple
    first_vector[1] = 3;

    println!("{:?}", first_vector);

    println!("Print some value: {}", first_vector[0]);

    println!("Get vector length: {}", first_vector.len());

    //vector opperations
    first_vector.push(21);
    println!("{:?}", first_vector);
    first_vector.pop();//remove last
    println!("{:?}", first_vector);

    //loop 
    for val in first_vector.iter(){
        println!("Loop val {}", val);
    }

    //mutate during interation

    for mut_val in first_vector.iter_mut(){
        *mut_val = *mut_val*3;
    }
    println!("After loop and mut {:?}", first_vector);

    //println!("Vector occupies {} bytes", std::mem::size_of_val(&first_vector));
    // ! simpler by use std::mem
    println!("Vector occupies {} bytes", mem::size_of_val(&first_vector));

    // How to slice?? - like other with ... 
    let slice = &first_vector[1..3];
    println!("Slice: {:?}", slice);
    let slice_first = &first_vector[..2];
    println!("Slice first: {:?}", slice_first);
    let slice_last = &first_vector[3..];
    println!("Slice last: {:?}", slice_last);


}