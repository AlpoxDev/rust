use regex::Regex;

fn main() {
    let regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    println!("Hello, world! {}", regex.is_match("2016-01-01"));
}
