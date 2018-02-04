extern crate select;
use select::predicate::{Predicate, Class};

type IplayerDocument = select::document::Document;

// type IplayerNode<'a> = select::node::Node<'a>;
pub struct IplayerNode<'a> {
    pub node: select::node::Node<'a>
}

impl<'a> IplayerNode<'a> {
    fn find_title(&self) -> String {
       self.node.find(Class("secondary").descendant(Class("title")))
           .next()
           .unwrap()
           .text()
    }
}

struct IplayerSelection<'a> {
    programme: Option<Programme<'a>>,
    prog_page: Option<String>,
}

impl<'a> IplayerSelection<'a> {
    fn new(inode: &IplayerNode) {

    }
}


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

#[cfg(test)]
mod tests {
   use super::*;
    use select::predicate::Name;

    #[test]
    fn test_document() {
        let doc =  IplayerDocument::from(include_str!("../testhtml/food1.html"));
        assert_eq!(&doc.find(Name("h1")).next().unwrap().text(), " Food  - A-Z ");
        let dn = doc.find(Class("list-item-inner")).next().unwrap();
        let inode  = IplayerNode { node: dn };
        assert_eq!(inode.find_title(), "The A to Z of TV Cooking");
    }
}
