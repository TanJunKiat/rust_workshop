fn main() {
    let x: u32 = 5;


    let mut my_string = String::from("uhmm");
    println!("my_string value is: {}",my_string);
    // i_own_stuff(my_string);
    // my_string is not accessible anymore after passing it through a function
    i_borrow_mut(&mut my_string);
    println!("my_string value is: {}",my_string);

    let ref_str = i_give_ref();
    println!("{}",ref_str);

    {
        let y = 2;
        // both x and y is accessible
    }
    // y is inaccessible
    // x is accessible
}

fn i_own_stuff(s: String){
    println!("{}",s);
}
fn i_borrow_stuff(s: &String){
    println!("{}",s);
}
fn i_borrow_mut(s: &mut String){
    s.push_str("... Yes, I mutated it.");
    println!("{}",s);
}

fn i_give_ref() -> &'static str {
    //let obtained_string = String::from("Obtained_data");
    //&obtained_string

    "obtained_data" // on binary, always available; static lifetime
}