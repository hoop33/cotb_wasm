mod utils;

use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const CIRCLE_DEGREES: i32 = 360;
const RGB_DIVISOR: f64 = 255.0;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, colors!");
}

#[wasm_bindgen]
pub fn spin(degrees: i32, color: &str) -> String {
    let spin_degrees = normalize_degrees(degrees);
    let (r, g, b) = hex_to_rgb(color);
    let (h, s, l) = rgb_to_hsl(r, g, b);
    "foo".into()
}

fn normalize_degrees(degrees: i32) -> i32 {
    match degrees {
        d if d < 0 => (degrees % CIRCLE_DEGREES) + CIRCLE_DEGREES,
        _ => degrees % CIRCLE_DEGREES,
    }
}

fn hex_to_rgb(color: &str) -> (i32, i32, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#[A-Fa-f0-9]{6}$").unwrap();
    }
    match RE.is_match(color) {
        true => {
            let rgb = i32::from_str_radix(&color[1..], 16).unwrap();
            (((rgb & 0xff0000) >> 16), ((rgb & 0x00ff00) >> 8), (rgb & 0x00ff))
        },
        false => (0, 0, 0),
    }
}

fn rgb_to_hsl(red: i32, green: i32, blue: i32) -> (i32, i32, i32) {
    let r: f64 = red as f64 / RGB_DIVISOR;
    let g: f64 = green as f64 / RGB_DIVISOR;
    let b: f64 = blue as f64 / RGB_DIVISOR;

    let min: f64 = r.min(g.min(b));
    let max: f64 = r.max(g.max(b));
    let range: f64 = max - min;

    let l: i32 = ((max + min) / 0.02) as i32;
    if range == 0.0 {
        return (0, 0, l);
    }

    let mut h: i32;
    if (r - max).abs() < std::f64::EPSILON {
        h = (60.0 * (((g - b) / range) % 6.0)) as i32;
    } else if (g - max).abs() < std::f64::EPSILON {
        h = (60.0 * (((b - r) / range) + 2.0)) as i32;
    } else {
        h = (60.0 * (((r - g) / range) + 4.0)) as i32;
    }
    if h < 0 {
        h += CIRCLE_DEGREES;
    }

    let s: i32;
    if l < 50 {
        s = (100.0 * (range / (max + min))) as i32;
    } else {
        s = (100.0 * (range / (2.0 - max - min))) as i32;
    }

    (h, s, l)
}

#[test]
fn normalize_degrees_should_return_0_when_360() {
    assert_eq!(0, normalize_degrees(CIRCLE_DEGREES));
}

#[test]
fn normalize_degrees_should_return_num_when_between_0_and_359() {
    for d in 0..CIRCLE_DEGREES {
        assert_eq!(d, normalize_degrees(d));
    }
}

#[test]
fn normalize_degrees_should_return_positive_when_negative() {
    for d in (-CIRCLE_DEGREES + 1)..0 {
        assert_eq!(d + CIRCLE_DEGREES, normalize_degrees(d));
    }
}

#[test]
fn normalize_degrees_should_wrap_when_greater_than_360() {
    for d in (CIRCLE_DEGREES + 1)..(CIRCLE_DEGREES * 2 - 1) {
        assert_eq!(d - CIRCLE_DEGREES, normalize_degrees(d));
    }
}

#[test]
fn hex_to_rgb_should_return_0_0_0_when_invalid_string() {
    assert_eq!((0, 0, 0), hex_to_rgb("foo"));
}

#[test]
fn hex_to_rgb_should_return_red_when_red() {
    assert_eq!((255, 0, 0), hex_to_rgb("#ff0000"));
}

#[test]
fn hex_to_rgb_should_return_green_when_green() {
    assert_eq!((0, 255, 0), hex_to_rgb("#00ff00"));
}

#[test]
fn hex_to_rgb_should_return_blue_when_blue() {
    assert_eq!((0, 0, 255), hex_to_rgb("#0000ff"));
}

#[test]
fn hex_to_rgb_should_return_proper_color() {
    assert_eq!((161, 178, 195), hex_to_rgb("#a1b2c3"));
}

#[test]
fn rgb_to_hsl_should_return_hsl_for_red() {
    assert_eq!((0, 100, 50), rgb_to_hsl(255, 0, 0));
}

#[test]
fn rgb_to_hsl_should_return_hsl_for_green() {
    assert_eq!((120, 100, 50), rgb_to_hsl(0, 255, 0));
}

#[test]
fn rgb_to_hsl_should_return_hsl_for_blue() {
    assert_eq!((240, 100, 50), rgb_to_hsl(0, 0, 255));
}

#[test]
fn rgb_to_hsl_should_return_hsl_for_color() {
    assert_eq!((210, 22, 69), rgb_to_hsl(161, 178, 195));
}
