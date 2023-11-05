#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::io::Read;


#[derive(Parser)]
struct  Cli {
    pattern: String,
    path: std::path::PathBuf,

}

fn main() -> io::Result<()> {
    let args = Cli::parse();    

    let file = File::open(&args.path)?;
    let mut reader = io::BufReader::new(file);

    let mut read = String::new();
    reader.read_to_string(&mut read);
    



    find_matches(&read , &args.pattern , &mut std:: io::stdout());

    Ok(())
   
}
 fn find_matches(read: &str , pattern : &str , mut writer: impl std::io::Write){

     for line in read.lines(){
        // let line = line?;
        if line.contains(pattern){
        writeln!(writer, "{}", line);
     }
    }
 }

    // unit test  to find a match
    #[test]
fn find_match(){
    let mut find = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem",&mut find);
    assert_eq!(find,b"lorem ipsum\n");
}