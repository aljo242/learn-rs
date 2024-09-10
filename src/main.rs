use ::std::fmt::Debug;
use std::any::type_name;

#[derive(Clone, Copy, Debug)]
struct Number {
    value: f64,
}

impl Number {
    fn print(self) {
        println!("Number: {}", self.value);
    }
}

impl Signed for Number {
    fn is_negative(&self) -> Option<bool> {
        Some(self.value < 0.0)
    }
}

trait Signed {
    fn is_negative(&self) -> Option<bool>;
}

type SignResult = Result<bool, &'static str>;

fn is_positive<T: Signed + Debug>(value: &T) -> SignResult {
    println!("is_positive: {}", type_name::<T>());

    match value.is_negative() {
        None => Err("unable to resolve"),
        Some(b) => Ok(b),
    }
}

fn main() {
    println!("Hello, world!");

    let n: Number = Number { value: 5.4 };

    match n.is_negative() {
        None => {
            panic!("failed to resolve");
        }
        Some(_b) => n.print(),
    }

    match is_positive(&n) {
        Err(why) => panic!("{}", why),
        Ok(_res) => n.print(),
    }

    let mut v1 = Vec::new();
    v1.push(n);
}
