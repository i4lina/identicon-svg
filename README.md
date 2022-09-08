# identicon-svg

Rust crate to quicly generate identicon avatar svgs.

Generate a new svg containing the identicon, given a size, a width and an hexadecimal string.

This crate is basically a port from [identicons](https://github.com/Zunawe/identicons), a Javascript library written by [Bryce Wilson](https://github.com/Zunawe).\

While the original library allows for various types of identicon, such as square, polygon and circle, this library allows only the square type. Another difference is that while the original library obtained the icon color from the hash, this crate uses [random_color](https://crates.io/crates/random_color/0.6.1) to generate a more bright and visually-pleasing color.

# Example

This will generate a new identicon of size 5x5. The color will be generated randomly.

```
use identicons_svg::generate;

let svg: String = generate("6a556d38357143305a4d6642724e45", 5, 128, None)
// svg will be a valid xml string, it could be saved to a file or displayed in a web page
```
