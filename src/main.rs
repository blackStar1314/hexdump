use std::env::args;
use std::fs::{OpenOptions};
use std::io::Write;

mod hexdump;
use hexdump::HexDumper;

fn main() {

    let arg1 = args().nth(1);
    let arg2 = args().nth(2);

    let filename = arg1.expect("Usage: hexdump.exe <filename> <filename>");
    let output_filename = arg2.expect("Usage: hexdump.exe <filename> <filename>");

    let mut dumper = HexDumper::new(&filename).expect("Unable open file");

    dumper.load().unwrap();

    let f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(output_filename);
    match f {
       Ok(mut file) => {
        let str = dumper.to_string();
        file.write_all(&str.as_bytes()).unwrap();
       },
       Err(err) => {
        println!("Dump file error : {}", err);
       } 
    }
}
