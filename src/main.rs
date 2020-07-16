mod lib;
use lib::lz_compress::*;

fn main() {
    println!("LZ77 implementation in Rust\n");

    let input = String::from("This is a test. This test is made for testing");
    
    println!("Input:\n{}\n", input);
    println!("Input length: {}\n", input.as_bytes().len());
    
    let output = lz77_compress(&input);

    //println!("Output (bytes): {:?}", output);
    println!("Compressed length: {}\n", output.len());


    println!("Decompressed:\n{}", lz77_decompress(&output));
    
}
