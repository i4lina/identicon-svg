use identicons_svg::{generate, Background, IdenticonOptions};

fn main() {
    let svg = vec![0; 500].iter().fold(String::new(), |acc, _i| {
        let svg = generate(IdenticonOptions::default());
        let svg2 = generate(IdenticonOptions {
            background: Background {
                color: "#ffffff".to_string(),
                ..Default::default()
            },
            ..Default::default()
        });
        acc + &svg
    });
}
