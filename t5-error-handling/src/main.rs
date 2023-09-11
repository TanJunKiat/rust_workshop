fn main() {
    play_with_errors();
}

enum CarryError {
    TooHeavy,
    TooFar,
    OtherError(String),
}

fn play_with_errors() {
    //i_panic();
    let request_result = carry_vec(60000,30000);
    
    match request_result {
        Ok(success) => println!("Request successful"),
        Err(e_vec) => {
            for error in e_vec.iter() {
                match error {
                    CarryError::TooHeavy => {println!("Carry request failed, too heavy.\nReduce load.")},
                    CarryError::TooFar => {println!("Carry request failed, too far.\nReduce distance.")},
                    _ => {}
                    }
                }
            }
        }

    // match request_result {
    //     Ok(success) => println!("Request successful"),
    //     Err(e) => match e {
    //         CarryError::TooHeavy => {println!("Carry request failed, too heavy.\nReduce load.")},
    //         CarryError::TooFar => {println!("Carry request failed, too far.\nReduce distance.")},
    //         _ => {}
    //     }
    // }


    //match request_result {
    //    Ok(success) => println!("Request successful"),
    //    Err(e) => println!("Error occured: {}",e)
    //}
    println!("Recovered")
}

fn i_panic() {
    panic!("Ahhh, that paniced me.") // Prints then terminate binary execution
}

// Result::{Err,Ok};
fn carry(weight: u32, distance: u32) -> Result<(), CarryError> {
    
    if weight > 50000 {
        // Error , too heavy
        return Result::Err(CarryError::TooHeavy);
    }
    if distance > 2000 {
        // Error, too far
        return Result::Err(CarryError::TooFar);
    }

    println!("Carried {} grams {} meters",weight,distance);
    Result::Ok(())
}

fn carry_vec(weight: u32, distance: u32) -> Result<(), Vec<CarryError>> {
    let mut errors = Vec::new();
    if weight > 50000 {
        // Error , too heavy
        errors.push(CarryError::TooHeavy);
    }
    if distance > 2000 {
        // Error, too far
        errors.push(CarryError::TooFar);
    }
    if errors.len() > 0 {
        return Err(errors);
    }
    println!("Carried {} grams {} meters",weight,distance);
    Result::Ok(())
}
