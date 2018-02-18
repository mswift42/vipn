#![feature(custom_attribute)]
extern crate select;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use select::predicate::{Predicate, Class, Name};
use chrono::prelude::*;

pub struct IplayerDocument {
    idoc: select::document::Document
}

impl IplayerDocument {
    pub fn programmes(&self) -> Vec<Programme> {
        self.idoc.find(Class("list-item-inner"))
            .map(|node| {
                let inode = IplayerNode { node };
                let prog = Programme::new(inode);
                prog
            })
            .collect()
    }
}

pub struct ProgrammeDB<'a> {
    pub categories: Vec<Category<'a>>,
    pub saved: DateTime<Utc>,
}

impl<'a> ProgrammeDB<'a> {
    pub fn new(cats: Vec<Category<'a>>) -> ProgrammeDB {
        ProgrammeDB {
            categories: cats,
            saved: Utc::now(),
        }
    }
}


struct MainCategoryDocument<'a> {
    idocs: Vec<&'a IplayerDocument>
}

pub struct Category<'a> {
    name: String,
    programmes: Vec<&'a Programme<'a>>,
}


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

    fn find_subtitle(&self) -> Option<String> {
        let sub = self.node.find(Class("secondary")
            .descendant(Class("subtitle")))
            .next();
        match sub {
            None => None,
            Some(txt) => Some(txt.text()),
        }
    }

    fn find_url(&self) -> String {
        let path = self.node.find(Name("a"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap_or("");
        if path.starts_with("http://www.bbc.co.uk") {
            path.to_string()
        } else {
            "http://www.bbc.co.uk".to_string() + path
        }
    }

    fn find_thumbnail(&self) -> &'a str {
        self.node.find(Class("rs-image")
            .descendant(Name("picture")
                .descendant(Name("source"))))
            .next()
            .unwrap()
            .attr("srcset")
            .unwrap_or("")
    }

    fn find_pid(&self) -> &'a str {
        match self.node.parent().unwrap().attr("data-ip-id") {
            None => self.node.find(Class("list-item-inner").descendant(Name("a")))
                .next()
                .unwrap()
                .attr("data-episode-id")
                .unwrap_or(""),
            Some(pid) => pid,
        }
    }

    fn find_synopsis(&self) -> String {
        self.node.find(Class("synopsis"))
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
    fn new(inode: IplayerNode) -> IplayerSelection {
        match inode.node.find(Class("view-more-container"))
            .next() {
            None => IplayerSelection {
                programme: Some(Programme::new(inode)),
                extra_prog_page: None,
            },
            Some(val) => IplayerSelection {
                programme: None,
                extra_prog_page: Some(val.attr("href").unwrap_or("")),
            },
        }
    }
}


#[derive(Debug)]
pub struct Programme<'a> {
    pub title: String,
    pub subtitle: Option<String>,
    pub synopsis: String,
    pub pid: &'a str,
    pub thumbnail: &'a str,
    pub url: String,
    pub index: usize,
}

impl<'a> Programme<'a> {
    fn new(inode: IplayerNode) -> Programme {
        let title = inode.find_title();
        let subtitle = inode.find_subtitle();
        let synopsis = inode.find_synopsis();
        let pid = inode.find_pid();
        let thumbnail = inode.find_thumbnail();
        let url = inode.find_url();
        let index = 0;
        Programme {
            title,
            subtitle,
            synopsis,
            pid,
            thumbnail,
            url,
            index,
        }
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use select::predicate::Name;

    #[rustfmt_skip]
    #[test]
    fn test_document() {
        let doc = select::document::Document::from(include_str!("../testhtml/food1.html"));
        let idoc = IplayerDocument { idoc: doc };
        assert_eq!(&idoc.idoc.find(Name("h1")).next().unwrap().text(), " Food  - A-Z ");
        let dn = &idoc.idoc.find(Class("list-item-inner")).next().unwrap();
        let inode = IplayerNode { node: *dn };
        assert_eq!(inode.find_title(), "The A to Z of TV Cooking");
        assert_eq!(inode.find_subtitle(), Some("Reversioned Series: 16. Letter P".to_string()));
        assert_eq!(inode.find_url(), "http://www.bbc.co.uk/iplayer/episode/b04w5mf0/the-a-to-z-of-tv-cooking-reversioned-series-16-letter-p".to_string());
        assert_eq!(inode.find_thumbnail(), "https://ichef.bbci.co.uk/images/ic/336x189/p02dd1vv.jpg".to_string());
        assert_eq!(inode.find_pid(), "b04vjm8d".to_string());
        let prog = Programme::new(inode);
        assert_eq!(prog.title, "The A to Z of TV Cooking");
        assert_eq!(prog.pid, "b04vjm8d");
        assert_eq!(prog.synopsis, "John Torode serves up a selection of cookery clips linked by the letter P.");
        //        let doc = select::document::Document::from(include_str!("../testhtml/food1.html"));
//        let idoc = IplayerDocument { idoc: doc };
        let programmes = idoc.programmes();
        assert_eq!(programmes.len(), 17);
        assert_eq!(programmes[0].title, "The A to Z of TV Cooking");
        assert_eq!(programmes[1].title, "Fanny Cradock Cooks for Christmas");
    }

    #[test]
    fn test_iplayer_selection() {
        let doc = select::document::Document::from(include_str!("../testhtml/food1.html"));
        let idoc = IplayerDocument { idoc: doc };
        let dn = &idoc.idoc.find(Class("list-item-inner")).next().unwrap();
        let inode = IplayerNode { node: *dn };
        assert_eq!(inode.find_title(), "The A to Z of TV Cooking");
        let isel = IplayerSelection::new(inode);
        let ip = isel.programme.unwrap();
        assert_eq!(ip.title, "The A to Z of TV Cooking");
    }
}
