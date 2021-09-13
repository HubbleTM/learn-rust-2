use std::ops::Add;
use std::fmt::{Display, Debug};

fn max<T: PartialOrd + Copy>(list: &[T]) -> T {
    if list.len() == 0 {
        panic!("empty list");
    }

    let mut max_val = list[0];

    for i in 1..list.len() {
        if max_val < list[i] {
            max_val = list[i];
        }
    }

    return max_val;
}

struct Point<T: Add + Copy> {
    x: T,
    y: T,
}

// Типаж
pub trait Sum<T: Add + Copy> {
    fn sum(&self) -> T;
}

impl<T: Add<Output=T> + Copy> Sum<T> for Point<T> {
    fn sum(&self) -> T {
        return self.x + self.y;
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

fn some_function_refactored<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("{}", p.sum())
}
