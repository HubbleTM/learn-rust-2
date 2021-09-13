use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct AliveStructure<'a> {
    field: &'a str,
}

impl<'a> AliveStructure<'a> {
    fn print(&self) {
        println!("{}", self.field);
    }
}

fn main() {
    let s1 = "abac".to_string();
    let res;

    {
        let s2 = "bbbaaa".to_string();
        res = longest(&s1, &s2);
        println!("{}", res);
    }

    // Will not compile cause s2 is unavailable.
    // println!("{}", res);

    let alive;
    {
        let s2 = "hahaha".to_string();
        alive = AliveStructure {
            field: &s2,
        }
    }

    // Will not compile cause s2 is unavailable.
    // println!("{}", alive.field)


    let alive = AliveStructure {
        field: "I am alive!",
    };

    alive.print();

    let s: &'static str = "I have a static lifetime.";
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
