# identicon-svg

Rust crate to quickly generate identicon avatar svgs.

Generate a new svg containing the identicon, given a size, a width and a hexadecimal string.

This crate is basically a port from [identicons](https://github.com/Zunawe/identicons), a Javascript library written by [Bryce Wilson](https://github.com/Zunawe).

While the original library allows for various types of identicon, such as square, polygon and circle, this library allows only the square type. Another difference is that while the original library obtained the icon color from the hash, this crate uses [random_color](https://crates.io/crates/random_color/0.6.1) to generate a more bright and visually-pleasing color.

# Example

This will generate a new identicon.
The size of the new svg will be between 4x4 and 8x8 (always a square), the width will be 128, the color will be randomly chosen.

Identicons are made from hex hashes and you can provide one yourself. This is not a means of encryption and you should not be using data relative to the user. That's why by default the hex hash is randomly generated. A function to create random hex hashes is also available.

```rust
use identicons_svg::generate;

let svg: String = generate(IdenticonOptions::default())
// svg will be a valid xml string, it could be saved to a file or displayed in a web page
```
