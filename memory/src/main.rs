use std::mem::drop;

fn is_strong<T: AsRef<str>>(pwd: T) -> bool {
    pwd.as_ref().len() > 5
}

fn main() {
    assert!(is_strong("Hello, world!"));
    assert!(is_strong(String::from("Hello, world!")));

    let a: i32 = 42;
    let b: Box<i32> = Box::new(84);
    let result = *b - a;

    let c = Box::new(1);
    let d = Box::new(1);
    let e = Box::new(1);

    let result1 = *c + *d + *e;
    drop(c);
    let f = Box::new(1);
    let result2 = *f + *d + *e;
    println!("{}, {}, {}", result, result1, result2);
}
