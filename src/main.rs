use identicons_svg::generate;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let svg = vec![0; 500].iter().fold(String::new(), |acc, _i| {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect();

        let mut rng = rand::thread_rng();

        dbg!(&hex::encode(&rand_string));

        let svg = generate(&hex::encode(&rand_string), rng.gen_range(4..8), 128, None);
        acc + &svg
    });
}
