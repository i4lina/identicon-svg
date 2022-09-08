use identicons_svg::{generate, IdenticonOptions};

fn main() {
    let svg = vec![0; 500].iter().fold(String::new(), |acc, _i| {
        let svg = generate(IdenticonOptions::default());
        let svg2 = generate(IdenticonOptions {
            color: String::from("#ffffff"),
            ..Default::default()
        });
        acc + &svg
    });
}
