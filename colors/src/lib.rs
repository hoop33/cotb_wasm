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

    let mut hspin = h + spin_degrees;
    if hspin > CIRCLE_DEGREES {
        hspin -= CIRCLE_DEGREES;
    }

    let (sr, sb, sg) = hsl_to_rgb(hspin, s, l);
    rgb_to_hex(sr, sb, sg)
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

fn hsl_to_rgb(h: i32, s: i32, l: i32) -> (i32, i32, i32) {
    let lightness: f64 = l as f64 / 100.0;
    let saturation: f64 = s as f64 / 100.0;
    let c: f64 = (1.0 - ((2.0 * lightness) - 1.0).abs()) * saturation;
    let x: f64 = c * (1.0 - (((h as f64 / 60.0) % 2.0) - 1.0).abs());
    let m: f64 = lightness - (c / 2.0);

    let (r, g, b) = match h {
        h if h < 60 => (c, x, 0.0),
        h if h < 120 => (x, c, 0.0),
        h if h < 180 => (0.0, c, x),
        h if h < 240 => (0.0, x, c),
        h if h < 300 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (((r + m) * 255.0) as i32, ((g + m) * 255.0) as i32, ((b + m) * 255.0) as i32)
}

fn rgb_to_hex(r: i32, g: i32, b: i32) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
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

#[test]
fn hsl_to_rgb_should_return_rgb_for_red() {
    assert_eq!((255, 0, 0), hsl_to_rgb(0, 100, 50));
}

#[test]
fn hsl_to_rgb_should_return_rgb_for_green() {
    assert_eq!((0, 255, 0), hsl_to_rgb(120, 100, 50));
}

#[test]
fn hsl_to_rgb_should_return_rgb_for_blue() {
    assert_eq!((0, 0, 255), hsl_to_rgb(240, 100, 50));
}

#[test]
fn hsl_to_rgb_should_return_rgb_for_color() {
    assert_eq!((158, 175, 193), hsl_to_rgb(210, 22, 69));
}

#[test]
fn rgb_to_hex_should_return_hex_when_red() {
    assert_eq!("#FF0000", rgb_to_hex(255, 0, 0));
}

#[test]
fn rgb_to_hex_should_return_hex_when_green() {
    assert_eq!("#00FF00", rgb_to_hex(0, 255, 0));
}

#[test]
fn rgb_to_hex_should_return_hex_when_blue() {
    assert_eq!("#0000FF", rgb_to_hex(0, 0, 255));
}

#[test]
fn rgb_to_hex_should_return_hex_when_color() {
    assert_eq!("#A1B2C3", rgb_to_hex(161, 178, 195));
}
