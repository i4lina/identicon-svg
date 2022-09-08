//! Generate a new svg containing the identicon, given a size, a width and an hexadecimal string.
//!
//! This crate is basically a port from [identicons](https://github.com/Zunawe/identicons), a Javascript library written by [Bryce Wilson](https://github.com/Zunawe).\
//!
//! While the original library allows for various types of identicon, such as square, polygon and circle, this library allows only the square type. Another difference is that while the original library obtained the icon color from the hash, this crate uses [random_color](https://crates.io/crates/random_color/0.6.1) to generate a more bright and visually-pleasing color.
//!
//!
//! # Example
//!
//! This will generate a new identicon of size 5x5. The color will be generated randomly.
//! ```
//! use identicons_svg::generate;
//!
//! let svg: String = generate("6a556d38357143305a4d6642724e45", 5, 128, None)
//! // svg will be a valid xml string, it could be saved to a file or displayed in a web page

use random_color::RandomColor;
use simple_xml_builder::XMLElement;

/// Generate the new identicon.
///
/// # Example
///
/// ```
/// let svg = generate("6a556d38357143305a4d6642724e45", 5, 128, None)
/// ```
///
/// # Panics
/// If the hash is not a valid hex string.
pub fn generate(hash: &str, size: u16, width: u16, color: Option<String>) -> String {
    let mut bytes = byte_array(hash);
    let mut bits = bit_array(bytes.clone());
    bytes.reverse();
    bits.reverse();

    let color = match color {
        None => RandomColor::new()
            //.hue(Color::Yellow)
            //.luminosity(Luminosity::Light)
            .to_hex(),
        Some(c) => c,
    };

    let box_width = width / (size + 1);
    let margin_width = (box_width / 2) + ((width % (size + 1)) / 2);

    let mut svg = XMLElement::new("svg");
    svg.add_attribute("widht", width);
    svg.add_attribute("height", width);
    svg.add_attribute("viewBox", format!("0 0 {0} {0}", width));
    svg.add_attribute("preserveAspectRatio", "xMinYMin");
    svg.add_attribute("xmlns", "http://www.w3.org/2000/svg");

    let mut map = vec![0u16; (size * size).into()];
    let mut i = 0;
    while i < size {
        let mut c = 0;
        while c < size / 2 {
            map[usize::from((i * size) + c)] = bits[usize::from((i * size) + c)];
            map[usize::from(((i + 1) * size) - (c + 1))] = bits[usize::from((i * size) + c)];
            c += 1
        }

        i += 1
    }

    let mut i = 0;

    while i < size * size {
        if map[usize::from(i)] == 1 {
            let r = i / size;
            let c = i % size;

            let mut child = XMLElement::new("rect");
            child.add_attribute("x", margin_width + (c * box_width));
            child.add_attribute("y", margin_width + (r * box_width));
            child.add_attribute("width", box_width);
            child.add_attribute("height", box_width);
            child.add_attribute("fill", color.clone());
            svg.add_child(child)
        }
        i += 1
    }

    svg.to_string()
}

fn byte_array(hash: &str) -> Vec<u16> {
    let mut bytes: Vec<u16> = vec![];

    let mut i = 0;
    while i < &hash.len() / 2 {
        let bytei = &hash.len() - ((i + 1) * 2);
        bytes.push(u16::from(
            u8::from_str_radix(&hash[bytei..bytei + 2], 16).unwrap(),
        ));
        i += 1
    }
    bytes
}

fn bit_string_from_u16(int: u16) -> String {
    let s = format!("00000000{int:b}");
    s[(s.len() - 8)..].to_string()
}

fn bit_array(a: Vec<u16>) -> Vec<u16> {
    let strings: Vec<String> = a.iter().map(|n| bit_string_from_u16(*n)).collect();
    let string = strings.join("");
    string
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u16>().unwrap())
        .collect()
}

#[cfg(feature = "show-icon")]
pub fn show_icon(icon: &str) {
    use std::{fs, path::PathBuf};

    static TEMP_FILE_PATH: &str = "temp323425.html";
    let edited = format!(
        r#"<!doctype html>
    <html>
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <script src="https://cdn.tailwindcss.com"></script>
      
    </head>
    <body>
    {0}
    </body>
    </html>"#,
        icon.clone()
    );

    fs::write(TEMP_FILE_PATH, edited).expect("Could not create temp file");

    let path = PathBuf::from(TEMP_FILE_PATH).canonicalize().unwrap();

    let path = match path.to_str() {
        Some(p) => p,
        None => panic!("could not generate temp path"),
    };

    webbrowser::open(path).unwrap();
}
