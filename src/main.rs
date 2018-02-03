extern crate select;

type IplayerDocument = select::document::Document;

pub struct Programme<'a> {
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub synopsis: &'a str,
    pub pid: &'a str,
    pub thumbnail: &'a str,
    pub url: &'a str,
    pub index: usize,
}

fn main() {
    println!("Hello, world!");
}
