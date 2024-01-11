extern crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::process::Output;
use std::time::Instant;


fn main(){
    if args().len()!=3{
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); // Open the file for reading
    let output = File::create(args().nth(2).unwrap()).unwrap(); // Open the file for writing
    let mut encoder = GzEncoder::new(output, Compression::default()); // Create a gzip encoder
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap(); // Compress the file from input to output
    let output = encoder.finish().unwrap(); // Finish the encoding
    print!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    print!(
        "Target len: {:?}",
        output.metadata().unwrap().len()
    );
    print!("Elapsed: {:?}", start.elapsed());
}