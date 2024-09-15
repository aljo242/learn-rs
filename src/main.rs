fn main() {
    println!("Hello, world!");
}

use num::Complex;

fn escape_time_check(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None
}

use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Result<(T, T), Error> {
    match s.find(separator) {
        None => Err(Error::new(ErrorKind::Other, "no separator found")),
        Some(index) => match (T::from_str(&s[0..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Ok((l, r)),
            _ => Err(Error::new(ErrorKind::Other, "unable to parse strings")),
        },
    }
}

#[test]
fn test_parse_pair() {
    assert!(parse_pair::<i32>("", ',').is_err(),);
    assert!(parse_pair::<i32>("10", ',').is_err(),);
    assert!(parse_pair::<i32>(",10", ',').is_err(),);
    assert!(parse_pair::<i32>("10,", ',').is_err(),);
    assert_eq!(
        parse_pair::<i32>("1,2", ',').expect("should return values"),
        (1, 2)
    );
    assert!(parse_pair::<i32>("1,2xz", ',').is_err(),);
    assert!(parse_pair::<i32>("1.5,2.5", ',').is_err(),);
    assert_eq!(
        parse_pair::<f64>("1.5,2.5", ',').expect("should return values"),
        (1.5, 2.5)
    );
}
