mod guessing_game;
mod news;
mod rectangle;
use news::{Summary, Tweet};
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

fn largest<T>(list: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    let first = list.first();
    match first {
        None => first,
        Some(n) => {
            let mut largest = n;
            for item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            Some(largest)
        }
    }
}

pub fn notify<T>(item: &T)
where
    T: Summary,
{
    println!("1 new tweet: {}", item.summarize());
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    match largest(&number_list) {
        None => println!("Number list is empty"),
        Some(n) => println!("The largest number is {}", n),
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };
    notify(&tweet);

    let path = Path::new("./hello.txt");
    let _uname = read_username_from_file(path);

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
