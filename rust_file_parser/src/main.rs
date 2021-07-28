//As of 07/27/2017: I have implemented a file parser which searches a text file against a search word
//                  and confirms whether or not the search word exists in the text file.


use std::fs::File;
use std::io::*;
use std::path::Path;


pub fn cleanse_string(s: &mut String) -> () {
    if Some('\n') == s.chars().next_back() {
          s.pop();
      }
    if Some('\r') == s.chars().next_back() {
          s.pop();
      }
}

pub fn main(){


    print!("Enter the name of the file in question: ");
    stdout().flush(); //flushing output stream
    let mut fname = String::new();
    stdin().read_line(&mut fname).expect("Not a valid string");

    print!("Enter the string to search for: ");
    stdout().flush(); //flushing output stream
    let mut goal = String::new();
    stdin().read_line(&mut goal).expect("Invalid string to parse for.");

    cleanse_string(&mut fname);
    cleanse_string(&mut goal);

    let fpath = Path::new(&fname);
    let file_display = fpath.display();

    let mut file = match File::open(&fpath){
        Ok(file) => file,
        Err(reason) => panic!("Couldn't open {} ... {}", file_display, reason),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Err(reason) => panic!("Couldn't read {}: {}", file_display, reason),
        Ok(_) => println!("{} read.", &fname),
    }
    stdout().flush(); //flushing output stream
    println!("\nThe contents of {x:} reads:\n\n{y:}", x = fname, y = contents);
    stdout().flush(); //flushing output stream

    match (contents.contains(&goal)){
        true => println!("{0} exists in {1}", goal, fname),
        false => println!("{0} does not exist in {1}", goal, fname),
    }
    stdout().flush(); //flushing output stream
}
