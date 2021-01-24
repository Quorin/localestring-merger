use crate::parse::{parse_data, read_file};

mod parse;
mod section;

fn main() {
    println!("Hello, world!");

    let file_data = read_file("./locale_string.txt");
    let data = parse_data(&*file_data);
    println!("{:#?}", data);
}
