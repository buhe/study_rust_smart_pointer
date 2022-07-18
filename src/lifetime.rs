// lifetime is bound checker
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


pub fn longest2<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

