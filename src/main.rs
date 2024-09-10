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

fn foreach_str<F>(f: F)
where
    F: Fn(&'static str),
{
    f("Earth");
    f("Mars");
    f("Pluto");
}

fn main() {
    println!("Hello, world!");

    let n: Number = Number { value: 5.4 };

    match n.is_negative() {
        None => {
            panic!("failed to resolve");
        }
        Some(b) => {
            if !b {
                n.print()
            }
        }
    }

    let mut vv = vec![n];
    if is_positive(&n).expect("failed to resolve") {
        n.print();
        vv.push(n);
    }

    foreach_str(|planet| println!("welcome to planet {}", planet));

    for i in  vv {
        i.print();
    }
}
