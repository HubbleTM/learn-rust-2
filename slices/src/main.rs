fn main() {
    let mut s = String::from("hello world");

    // [starting_index..ending_index]
    let hello = &s[0..5];
    let world = &s[6..11];

    // immutable borrow occurs here
    let word = first_word(&s);

    // mutable borrow - cannot edit string
    // s.clear();

    // immutable borrow later used here
    println!("The first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}