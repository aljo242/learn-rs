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

use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match(T::from_str(&s[0..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("1,2", ','), Some((1,2)));
    assert_eq!(parse_pair::<i32>("1,2xz", ','), None);
    assert_eq!(parse_pair::<i32>("1.5,2.5", ','), None);
    assert_eq!(parse_pair::<f64>("1.5,2.5", ','), Some((1.5,2.5)));
}