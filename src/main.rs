mod guessing_game;
mod rectangle;
use rectangle::Rectangle;
use std::collections::HashMap;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn read_username_from_file(path: &Path) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    let path = Path::new("./hello.txt");
    let uname = read_username_from_file(path);

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{}", s);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    guessing_game::run_game();
    let square = Rectangle::square(30);
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold square? {}", rect.can_hold(&square));

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
