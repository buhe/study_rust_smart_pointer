use std::{ops::Deref, f32, fmt::Debug};

use crate::{generic::Point, lifetime::{longest, longest2}, thread::join1};
mod generic;
mod lifetime;
mod smart_pointer;
mod thread;
impl<T: Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("{:#?} leave.", self.0);
    }
}
impl<T: Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T: Debug>(T);

impl<T: Debug> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn use_ref(_r: Box<i32>) {

}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    let z = Box::new(3);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    use_ref(z);

    drop(y);

    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let f = Point{x: 5.1 as f32, y: 6.1 as f32};

    println!("f.d = {}", f.distance_from_origin());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    longest2(string1.as_str(), string2);

    join1();
}
