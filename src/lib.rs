//! Generate a new svg containing the identicon, given a size, a width and an hexadecimal string.
//!
//! This crate is basically a port from [identicons](https://github.com/Zunawe/identicons), a Javascript library written by [Bryce Wilson](https://github.com/Zunawe).\
//!
//! While the original library allows for various types of identicon, such as square, polygon and circle, this library allows only the square type. Another difference is that while the original library obtained the icon color from the hash, this crate uses [random_color](https://crates.io/crates/random_color/0.6.1) to generate a more bright and visually-pleasing color.
//!
//!
//! # Example
//!
//! This will generate a new identicon.\
//! The size of the new svg will be between 4x4 and 8x8 (always a square), the width will be 128, the color will be randomly chosen.\
//!
//! Identicon are made from hex hashes and you can provide one yourself. This is not a mean of encryption and you should not be using data relative to the user. That's why by default the hex hash is randomly generated. A function to create random hex hashes is also avalible.
//! ```
//! use identicons_svg::generate;
//!
//! let svg: String = generate(IdenticonOptions::default())
//! // svg will be a valid xml string, it could be saved to a file or displayed in a web page
//!```
//!
//! # Optional features
//!
//! - **`show-icon`** - Function `show_icon(icon: &str)` to save the svg in a temporary file and open it in a tab of the default browser

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use random_color::RandomColor;
use simple_xml_builder::XMLElement;

/// Generate the new identicon.
///
/// # Example
///
/// ```
/// let svg = generate(IdenticonOptions {color: "#a97896", width: 50, ..Default::default()})
/// ```
///
/// # Panics
/// If the hash is not a valid hex string.
pub fn generate(options: IdenticonOptions) -> String {
    let IdenticonOptions {
        hash,
        color,
        width,
        size,
        background,
    } = options;
    let mut bytes = byte_array(&hash);
    let mut bits = bit_array(bytes.clone());
    bytes.reverse();
    bits.reverse();

    let box_width = width / (size + 1);
    let margin_width = (box_width / 2) + ((width % (size + 1)) / 2);

    let mut svg = XMLElement::new("svg");
    svg.add_attribute("widht", width);
    svg.add_attribute("height", width);
    svg.add_attribute("viewBox", format!("0 0 {0} {0}", width));
    svg.add_attribute("preserveAspectRatio", "xMinYMin");
    svg.add_attribute("xmlns", "http://www.w3.org/2000/svg");

    svg.add_child(background.to_elem(width));

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

/// Creates a hex string from a random string that can be used for generating an identicon
pub fn new_hash(len: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    hex::encode(&rand_string)
}

/// Is the type for the background field of IdenticonOptions. Implements `Default`: if not otherwise specified the identicon will have a background of color `rgb(240, 240, 240)` and will not have rounded angles.
///
/// # Example
///
/// ```
/// use identicons_svg::{generate, IdenticonOptions, Background};
///
/// let svg = generate(IdenticonOptions {
///     background: Background {
///         color: "#8615b9",
///         ..Default::default()    
///         },
///         ..Default::default()   
/// })
/// ```
pub struct Background {
    /// Rounded angles
    pub r: u16,
    /// A valid color string for svg (eg. `"#ffffff"`)
    pub color: String,
}

impl Default for Background {
    fn default() -> Self {
        Background {
            r: 0,
            color: "rgb(240, 240, 240)".to_string(),
        }
    }
}

impl Background {
    fn to_elem(&self, width: u16) -> XMLElement {
        let mut child = XMLElement::new("rect");
        child.add_attribute("x", 0);
        child.add_attribute("y", 0);
        child.add_attribute("width", width);
        child.add_attribute("height", width);
        child.add_attribute("rx", &self.r);
        child.add_attribute("ry", &self.r);
        child.add_attribute("fill", &self.color);
        child
    }
}

/// Is the argument for the generate function. It implements `Default`, for an opinionated but quick identicon.
pub struct IdenticonOptions {
    /// The lenght of the side of the square that is the identicon
    pub size: u16,
    /// A valid color string for svg (eg. `"#ffffff"`)
    pub color: String,
    /// Width in pixel of the identicon
    pub width: u16,
    /// A valid hex string from witch to generate the identicon
    pub hash: String,
    /// Edit the background of the identicon, default is provided.
    pub background: Background,
}

impl Default for IdenticonOptions {
    /// Uses [rand](https://crates.io/crates/rand) and [hex](https://crates.io/crates/hex) to generate a valid hash. The size is selected randomly in a range from 4 to 8. The color is generated with [random_color](https://crates.io/crates/random_color/0.6.1) and `width = 128`.
    fn default() -> Self {
        let color = RandomColor::new()
            //.hue(Color::Yellow)
            //.luminosity(Luminosity::Light)
            .to_hex();

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect();

        let hash = hex::encode(&rand_string);

        let mut rng = rand::thread_rng();

        IdenticonOptions {
            size: rng.gen_range(4..8),
            color,
            width: 128,
            hash,
            background: Background::default(),
        }
    }
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

/// This function is avalible under the **`show-icon`** feature flag.\
/// This will save the given svg string in an html file and than open it in a default browser's tab, using [webbrowser](https://crates.io/crates/webbrowser)
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
