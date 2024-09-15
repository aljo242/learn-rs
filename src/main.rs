use image::ImageBuffer;
use num::Complex;
use std::env;
use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandle.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right");

    write_buffer_to_image(&args[1], bounds, upper_left, lower_right)
        .expect("expected valid render");
}

fn write_buffer_to_image(
    filename: &str,
    bounds: (u32, u32),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> image::ImageResult<()> {
    let buffer = ImageBuffer::from_fn(bounds.0, bounds.1, |x, y| {
        let point = map_pixel_to_point(bounds, (x, y), upper_left, lower_right);

        match escape_time_check(point, 255) {
            None => image::Rgb([0u8, 0u8, 0u8]),
            Some(count) => {
                let value = 255 - count as u8;
                image::Rgb([value, value, value])
            }
        }
    });

    buffer.save_with_format(filename, image::ImageFormat::Png)?;

    Ok(())
}

fn map_pixel_to_point(
    bounds: (u32, u32),
    pixel: (u32, u32),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn escape_time_check(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Result<(T, T), Error> {
    match s.find(separator) {
        None => Err(Error::new(ErrorKind::Other, "no separator found")),
        Some(index) => match (T::from_str(&s[0..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Ok((l, r)),
            _ => Err(Error::new(ErrorKind::Other, "unable to parse strings")),
        },
    }
}

fn parse_complex(s: &str) -> Result<Complex<f64>, Error> {
    match parse_pair(s, ',') {
        Ok((re, im)) => Ok(Complex { re, im }),
        Err(e) => Err(e),
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

#[test]
fn test_parse_complex() {
    assert!(parse_complex("").is_err(),);
    assert!(parse_complex("10").is_err(),);
    assert!(parse_complex(",10").is_err(),);
    assert!(parse_complex("10,").is_err(),);

    assert_eq!(
        parse_complex("1.0,10.3").expect("should return value"),
        Complex { re: 1.0, im: 10.3 }
    )
}

#[test]
fn test_map_pixel_to_point() {
    assert_eq!(
        map_pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    )
}
