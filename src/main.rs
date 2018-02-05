extern crate select;
use select::predicate::{Predicate, Class, Name};

type IplayerDocument = select::document::Document;

// type IplayerNode<'a> = select::node::Node<'a>;
pub struct IplayerNode<'a> {
    pub node: select::node::Node<'a>
}

impl<'a> IplayerNode<'a> {
    fn find_title(&self) -> &'a str {
       self.node.find(Class("secondary").descendant(Class("title")))
           .next()
           .unwrap()
           .text()
           .as_ref()
    }

    fn find_subtitle(&self) -> Option<&'a str> {
        let sub = self.node.find(Class("secondary").descendant(Class("subtitle")))
            .next();
        match sub {
            None => None,
            Some(txt) => Some(txt.text().as_ref()),
        }
    }

    fn find_url(&self) -> &'a str {
        let path = self.node.find(Name("a"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        if path.starts_with("http://www.bbc.co.uk") {
            path
        } else {
            &(String::from("http://www.bbc.co.uk") + path)
        }
    }

    fn find_thumbnail(&self) -> &'a str {
        self.node.find(Class("rs-image")
            .descendant(Name("pictrue")
                .descendant(Name("source"))))
            .next()
            .unwrap()
            .attr("srcset")
            .unwrap_or("")
    }

    fn find_pid(&self) -> &'a str {
        match self.node.attr("data-ip-id") {
            None => self.node.find(Class("list-item-inner").descendant(Name("a")))
                .next()
                .unwrap()
                .attr("data-episode-id")
                .unwrap(),
            Some(pid) => pid,
        }
    }

    fn find_synopsis(&self) -> &'a str {
        self.node.find(Class("synopsis"))
            .next()
            .unwrap()
            .text()
            .as_ref()
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
                extra_prog_page: Some(extra_prog_page)
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
