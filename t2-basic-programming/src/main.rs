fn main() {
    println!("Hello, world!");
    //working_with_variables();
    //data_types();
    //function_printer("how are you");
    //println!("{}",function_greeting("Jon"));
    //println!("Number is {}",play_if_pretty(256));
    play_with_loops();
}

fn working_with_variables(){
    let mut x = 5;
    x = 6; // Throw error if there is no mut infront of x
    
    println!("x is {}",x);
    let y = 5; 
    println!("y is {}",y);
    let y = 6; // y is defined
    println!("y is {} now",y);
}
fn data_types(){
    let x: u32 = 5;
    let y: i128 = -4;
    let z: bool = true;
    let string_slice: &str = "I'm a str"; // not mutable
    let string_full: String = String::from("I'm a string");

    print_type_of(&x);
    print_type_of(&y);
    print_type_of(&z);
    print_type_of(&string_slice);
    print_type_of(&string_full);

}
fn function_printer(p1: &str){
    println!("{}",p1);
}
fn function_greeting (name: &str) -> String {
    let mut greet = String::from("Hi, I am ");
    greet.push_str(name);
    greet // no semi-colon means return, or you can write
    //return greet;
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>()) // print data type
}
fn play_if(num: i32) -> String{

    let mut category = String::from("");

    if num > 250 {
        println!("That's a big one there mate.");
        category = String::from("High");
    }
    else if num > 125 {
        println!("That's a fair one.");
        category = String::from("Medium");
    }
    else {
        println!("That's a small one there mate.");
        category = String::from("Low");
    }

    category

}
fn play_if_pretty(num: i32) -> String{
    if num > 250 {
        String::from("high")
    }
    else if num > 125 {
        String::from("medium")
    }
    else {
        String::from("low")
    }
}
fn play_with_loops(){
    for num in 1..5 {   // 1 to 5, not inclusive of 5
        println!("FOR loop is at {}",num);
    }
    let mut iterations = 7;
    while iterations > 0 {
        iterations -= 1;
        println!("WHILE loop is at {}",iterations);
    }

    let mut tracker = 0;
    loop {
        tracker += 1;
        println!("LOOP is at {}",tracker);
        if tracker > 4 {
            break;
        }
    }
}

