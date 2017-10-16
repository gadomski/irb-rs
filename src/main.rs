#[macro_use]
extern crate clap;
extern crate irb;

use clap::App;
use irb::Irb;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("info") {
        let irb = Irb::from_path(matches.value_of("FILE").unwrap()).expect("Could not open file");
        println!("Image dimensions:");
        println!("  Width: {}", irb.image_width().unwrap());
        println!("  Height: {}", irb.image_height().unwrap());
    }
}
