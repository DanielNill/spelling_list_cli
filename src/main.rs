use std::env;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() > 2 {
        println!("spl only accepts 1 word at a time");
    }
    else {
        let file = match OpenOptions::new().write(true).open("spelling_list.txt") {
            Ok(file) => file,
            Err(..) => panic!("Error recording word"),
        };
        let mut writer = BufWriter::new(&file);
        writer.write_all(args[1].as_bytes()).expect("Error recording word 1");
        println!("{} added to your spelling list", args[1]); 
    }
}