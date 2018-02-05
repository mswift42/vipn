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
    extra_prog_page: Option<&'a str>,
}

impl<'a> IplayerSelection<'a> {
    fn new(inode: &IplayerNode) -> IplayerSelection<'a> {
        let extra_prog_page = inode.node.find(Class("view-more-container"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        if extra_prog_page != "" {
            IplayerSelection {
                programme: None,
                extra_prog_page: Some(extra_prog_page.to_string())
            }
        } else {
            IplayerSelection {
                programme: Some(Programme{}),
                extra_prog_page: None
            }
        }
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

impl<'a> Programme<'a> {
    fn new(inode: &IplayerNode) {

    }
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
