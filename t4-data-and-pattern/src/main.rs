// Declare structures
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,

}


// Traits defines standard behaviour for structures
trait Describe {
    fn describe(self: &Self) -> String;

    fn default_implementation(){
        println!("Default implementated function");
    }
}

// Functions for structures
impl Person {
    fn intro(&self) {
            println!("Hello,\nI'm {} and I'm {} years old.",self.name,self.age);
    }

    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn type_as_string() -> String {
        "Person".into()
    }
}

impl Describe for Person {
    fn describe(self: &Self) -> String{
        format!("Person({},{})",self.name, self.age)
    }
}


// trait DoAction {
//     fn run(self){
//         printl1!("Running");
//         self.run_inner();
//     }

//     fn run_inner(self);
// }


// struct CoinBalance {}

// impl CoinBalance {
//     fn add(self, balance_to_add: Self){ // Self implies that the type must match

//     }
// }

fn playing_with_struct() {
    let mut person = Person {
        name: "Jon Doe".into(),
        age: 34,
    };
    //println!("Person {} - {} years",person.name,person.age);
    println!("Person {:#?}",person);

    person.intro();
    person.rename("John".into());

    println!("{}", Person::type_as_string());

    println!("{}", person.describe());
    place_order(person);

}

fn place_order(user: Person) {
    println!("{} placed an order",user.name)
}


//
fn main(){
    //playing_with_struct();
    playing_with_enums();
}

enum RobotEvent {
    WallAhead(u32),
    RequestCarry(String),
    AdminCommand(u128),
}

fn playing_with_enums() {
    let event = RobotEvent::WallAhead(50);
    robot_run_event(event);
    let event = RobotEvent::WallAhead(5);
    robot_run_event(event);
    let event = RobotEvent::RequestCarry("Room 5".into());
    robot_run_event(event)
}

fn robot_run_event(event: RobotEvent) {
    match event {
        RobotEvent::WallAhead(distance) => {
            if distance < 20 {
                println!("Turning.");
            }
            else {
                println!("Going straight.");
            }
        }
        RobotEvent::RequestCarry(location) => {
            println!("Carry to {}",location);
        }
        _ => {} // to cover all cases; whatever else
    }
}