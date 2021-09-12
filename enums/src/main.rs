enum Countries {
    UKRAINE,
    USA,
    JAPAN,
}

enum IP {
    V4(String),
    V6(String),
}

#[derive(PartialEq)]
enum IP2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IP2 {
    fn get_address(&self) -> String {
        return match self {
            IP2::V4(i1, i2, i3, i4) => format!("{} {} {} {}", i1, i2, i3, i4),
            IP2::V6(i) => String::from(i)
        };
    }
}

fn main() {
    let country = Countries::UKRAINE;
    travel(Countries::USA);

    route(IP::V4("192.168.1.1".to_string()));
    route2(IP2::V4(192, 168, 1, 1));

    println!("{}", IP2::V4(192, 168, 1, 1).get_address());
    println!("{}", IP2::V6("::1".to_string()).get_address());


    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    match absent_number {
        Some(i) => println!("Number = {}", i),
        None => println!("Absent number!")
    }

    match some_number {
        Some(i) => println!("Number = {}", i),
        None => println!("Absent number!")
    }

    if let Some(str) = some_string {
        println!("Some string = {}", str);
    }
}

fn travel(country: Countries) {}

fn route(ip: IP) {}

fn route2(ip: IP2) {}