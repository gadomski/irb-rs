extern crate clap;
extern crate irb;

use clap::{App, Arg, SubCommand};
use irb::text::File;

fn main() {
    let matches = App::new("InfraTec .irb files")
        .author("Pete Gadomski <pete@gadom.ski>")
        .subcommand(SubCommand::with_name("query")
                        .about("query the image value at a given pixel")
                        .arg(Arg::with_name("INFILE")
                                 .help("image file path")
                                 .required(true)
                                 .index(1))
                        .arg(Arg::with_name("X")
                                 .help("x coordinate to query")
                                 .required(true)
                                 .index(2))
                        .arg(Arg::with_name("Y")
                                 .help("y coordinate to query")
                                 .required(true)
                                 .index(3)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("query") {
        let infile =
            File::open(matches.value_of("INFILE").unwrap()).expect("Unable to open infile");
        let image = infile.into_image().expect("Unable to read file into image");
        let x: usize = matches.value_of("X")
            .unwrap()
            .parse()
            .unwrap();
        let y: usize = matches.value_of("Y")
            .unwrap()
            .parse()
            .unwrap();
        if x < image.width && y < image.height {
            println!("{}", image[(y, x)]);
        } else {
            panic!("Coordinates ({}, {}) out of image dimensions {} x {}",
                   y,
                   x,
                   image.height,
                   image.width);
        }
    }
}
