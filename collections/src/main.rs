use std::collections::HashMap;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v1.push(1);
    v1.push(2);
    v1.push(3);

    for i in 0..v1.len() {
        print!("{} ", v1[i]);
    } // 1 2 3
    println!();

    v1.pop();
    for i in 0..v1.len() {
        print!("{} ", v1[i]);
    } // 1 2
    println!();

    // !Will cause panic
    // let not_exist = &v1[10];

    let not_exist = v1.get(10);
    if let None = not_exist {
        println!("None");
    } // None


    for value in &mut v1 {
        *value += 1;
    }

    for value in v1 {
        print!("{} ", value);
    } // 1 2
    println!();

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Cannot do that cause of borrowing s2 in s3;
    // println!("{} {}", s2, s1);

    for c in s3.chars() {
        print!("{} ", c);
    } // H e l l o ,   w o r l d !
    println!();

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    for (k, v) in &map {
        println!("{} {}", k, v);
    }

    map.entry(1).or_insert("one".to_string());
    println!("{:?}", map);
}
