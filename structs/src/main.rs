#[derive(Clone)]
struct User {
    first_name: String,
    last_name: String,
    mail: String,
    age: u16,
}

impl User {
    fn full_name(&self) -> String {
        let mut full_name = String::from(&self.first_name);
        full_name.push_str(" ");
        full_name.push_str(&self.last_name);
        full_name
    }

    fn new() -> User {
        User {
            first_name: "".to_string(),
            last_name: "".to_string(),
            mail: "".to_string(),
            age: 0,
        }
    }
}

// tuple structs
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        mail: String::from("someone@example.com"),
        first_name: String::from("Oleg"),
        last_name: String::from("Fomenko"),
        age: 99,
    };


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", user1.full_name());

    let user2 = User {
        mail: String::from("someone@example.com"),
        // need clone for coping strings
        ..user1.clone()
    };

    println!("{}", user1.full_name());

    let user3 = User::new();
    println!("{}", user3.full_name());
}
