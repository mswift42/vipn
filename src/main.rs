#![feature(custom_attribute)]
extern crate reqwest;
extern crate select;
extern crate serde;
extern crate serde_json;
extern crate url;

use select::predicate::{Class, Name, Predicate};
use std::thread;
use url::{Url, ParseError};

pub struct IplayerDocument {
    idoc: select::document::Document,
}

pub struct  BeebURL<'a> {
    url: &'a str,
}



pub struct TestHTMLURL<'a> {
    url: &'a str,
}


impl<'a> BeebURL<'a> {
    pub fn load_document(&self) -> IplayerDocument {
        let uri = Url::parse(self.url).unwrap();
       let resp = reqwest::get(uri);
        match resp {
           Err(e) => panic!(e),
            Ok(body) => {
                let doc = select::document::Document::from_read(body);
                match doc {
                    Err(e) => panic!(e),
                    Ok(iplayerdoc) => IplayerDocument {idoc: iplayerdoc },
                }
            }
        }
    }
}

impl<'a> TestHTMLURL<'a> {
    pub fn load_document(&self) -> IplayerDocument {
       IplayerDocument {idoc: select::document::Document::from(self.url) }
    }
}

pub trait DocumentLoader {
    fn load_document(&self) -> IplayerDocument;
}
impl IplayerDocument {
    fn selection_results(&self) -> Vec<IplayerSelection> {
        self.idoc
            .find(Class("list-item-inner"))
            .map(|node| {
                let inode = IplayerNode { node };
                IplayerSelection::new(inode)
            })
            .collect()
    }

    fn next_pages(&self) -> Vec<&str> {
        self.idoc
            .find(Class("page").descendant(Name("a")))
            .map(|node| node.attr("href").unwrap_or(""))
            .collect()
    }

}

pub struct ProgramPage {
    doc: IplayerDocument,
}

impl<'a> ProgramPage {
    // pub fn new(&self) -> Vec<&'a Programme> {
    //     let node = self.doc.idoc.find(Class("content-item")).next().unwrap();
    //     let title = node.find(Class("hero-header__title"))
    //         .next().unwrap();
    //     let subtitle = node.find(Class("content-item__title")).next().unwrap();
    //     let synopsis = node.find(Class("content-item__info__secondary").
    //         descendant(Class("content-item__description"))).next().unwrap();
    //     let set = self.doc.idoc.find(Name("source")).next().unwrap()
    //     .attr("srcset").unwrap_or("");
    //     let split = set.split_whitespace();
    //     let thumb = split.next().unwrap();
    // }
//    pub fn new(&self) -> Vec<&'a Programme> {
//        let node = self.doc.idoc.find(Class("content-item")).next().unwrap();
//        let inode = IplayerNode{node};
//        let title = node.find(Class("hero-header__title"))
//            .next().unwrap();
//        let subtitle = node.find(Class("content-item__title")).next().unwrap();
//        let synopsis = node.find(Class("content-item__info__secondary").
//            descendant(Class("content-item__description"))).next().unwrap();
//        let thumbnail = inode.find_thumbnail();
//        let url = inode.find_url();
//        }
//    }
}


pub struct ProgrammeDB<'a> {
    pub categories: Vec<Category<'a>>,
    pub saved: std::time::SystemTime,
}

impl<'a> ProgrammeDB<'a> {
    pub fn new(cats: Vec<Category<'a>>) -> ProgrammeDB<'a> {
        ProgrammeDB {
            categories: cats,
            saved: std::time::SystemTime::now(),
        }
    }
}

struct MainCategoryDocument<'a> {
    maindoc: &'a IplayerDocument,
    nextdocs: Vec<&'a IplayerDocument>,
    selectionresults: Vec<&'a IplayerSelection<'a>>,
}

impl<'a> MainCategoryDocument<'a> {
    fn all_docs(&self) -> Vec<&'a IplayerDocument> {
        let mut res: Vec<&'a IplayerDocument> = vec![self.maindoc];
        for i in &self.nextdocs {
            res.push(i);
        }
        res
    }
    fn selection_results(&self) -> Vec<IplayerSelection> {
        let all_docs = self.all_docs();
        all_docs
            .iter()
            .flat_map(|idoc| idoc.selection_results())
            .collect()
    }
    fn programmes(&self) -> Vec<Programme> {
        let selection_results = self.selection_results();
        let mut progs: Vec<Programme> = vec![];
        for selres in selection_results {
            if let Some(prog) = selres.programme {
                progs.push(prog);
            }
        }
        progs
    }

    fn extra_program_pages(&self) -> Vec<&str> {
        let selection_results = self.selection_results();
        let mut pages: Vec<&str> = vec![];
        for selres in selection_results {
            if let Some(page) = selres.extra_prog_page {
                pages.push(page);
            }
        }
        pages
    }

    fn next_pages(&self) -> Vec<&str> {
        self.maindoc.next_pages()
    }
}

pub struct Category<'a> {
    name: String,
    programmes: Vec<Programme<'a>>,
}

impl<'a> Category<'a> {
    pub fn new(name: String, programmes: Vec<Programme<'a>>) -> Category<'a> {
        Category { name, programmes }
    }
}

// type IplayerNode<'a> = select::ode::Node<'a>;
pub struct IplayerNode<'a> {
    pub node: select::node::Node<'a>,
}

impl<'a> IplayerNode<'a> {
    fn find_title(&self) -> String {
        self.node
            .find(Class("content-item__title"))
            .next()
            .unwrap()
            .text()
    }

    fn find_subtitle(&self) -> Option<String> {
        let sub = self.node
            .find(Class("content-item__info-primary")
                .descendant(Class("content-item__description")))
            .next();
        match sub {
            None => None,
            Some(txt) => Some(txt.text()),
        }
    }

    fn find_url(&self) -> String {
        let path = self.node
            .find(Name("a"))
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
        self.node
            .find(Class("rs-image").descendant(Name("picture").descendant(Name("source"))))
            .next()
            .unwrap()
            .attr("srcset")
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap()
    }

    fn find_synopsis(&self) -> String {
        self.node.find(Class("content-item__info__secondary")
            .descendant(Class("content-item__description"))).next().unwrap().text()
    }
}

struct IplayerSelection<'a> {
    programme: Option<Programme<'a>>,
    extra_prog_page: Option<&'a str>,
}

impl<'a> IplayerSelection<'a> {
    fn new(inode: IplayerNode) -> IplayerSelection {
        match inode.node.find(Class("view-more-container")).next() {
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
    pub thumbnail: &'a str,
    pub url: String,
    pub index: usize,
}

impl<'a> Programme<'a> {
    fn new(inode: IplayerNode) -> Programme {
        let title = inode.find_title();
        let subtitle = inode.find_subtitle();
        let synopsis = inode.find_synopsis();
        let thumbnail = inode.find_thumbnail();
        let url = inode.find_url();
        let index = 0;
        Programme {
            title,
            subtitle,
            synopsis,
            thumbnail,
            url,
            index,
        }
    }
    fn new_from_program_page(inode: IplayerNode, title: String) -> Programme {
        // let subtitle = match inode.node.find(Class("content-item__title")).next() {
        //     None => None,
        //     Some(text) => text.to_string();
        // };
        let subtitle = inode.find_subtitle();
        let synopsis = inode.node.find(Class("content-item__info__secondary").
            descendant(Class("content-item__description"))).next().unwrap().text();
        let url = inode.node.find(Name("a")).next().unwrap().
            attr("href").unwrap_or("").to_string();
        let thumbnail = inode.node.find(Class("rs-image").descendant(Class("picture")
            .descendant(Class("source")))).next().unwrap().attr("srcset").unwrap_or("");
        let index = 0;
        Programme {
            title,
            subtitle,
            synopsis,
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
        assert_eq!(&idoc.idoc.find(Name("h1")).next().unwrap().text(), "Food");
        let dn = &idoc.idoc.find(Class("content-item")).next().unwrap();
        let inode = IplayerNode { node: *dn };
        assert_eq!(inode.find_title(), "The Big Crash Diet");
        assert_eq!(inode.find_subtitle(), Some("Reversioned Series: 16. Letter P".to_string()));
        assert_eq!(inode.find_url(), "http://www.bbc.co.uk/iplayer/episode/b04w5mf0/the-a-to-z-of-tv-cooking-reversioned-series-16-letter-p".to_string());
        assert_eq!(inode.find_thumbnail(), "https://ichef.bbci.co.uk/images/ic/336x189/p02dd1vv.jpg".to_string());
        let prog = Programme::new(inode);
        assert_eq!(prog.title, "The A to Z of TV Cooking");
        assert_eq!(prog.synopsis, "John Torode serves up a selection of cookery clips linked by the letter P.");
    }

    //#[test]
    fn test_iplayer_selections() {
        let idoc = IplayerDocument {
            idoc: select::document::Document::from(include_str!("../testhtml/food1.html")),
        };
        let sels = idoc.selection_results();
        assert_eq!(sels.len(), 17);
        let prog1_page = sels[1].extra_prog_page.unwrap();
        assert!(sels[1].programme.is_none());
        assert_eq!(prog1_page, "/iplayer/episodes/p05jv04g");
        let prog17_page = sels[16].extra_prog_page.unwrap();
        assert_eq!(prog17_page, "/iplayer/episodes/b09l5mdv");
        assert!(sels[16].programme.is_none());
        let prog16_page = sels[15].extra_prog_page.unwrap();
        assert_eq!(prog16_page, "/iplayer/episodes/b07x182s");
        assert!(sels[15].programme.is_none());
    }

    // #[test]
    fn test_programmes() {
        let idoc = IplayerDocument {
            idoc: select::document::Document::from(include_str!("../testhtml/food1.html")),
        };
        let mcd = MainCategoryDocument {
            maindoc: &idoc,
            selectionresults: vec![],
            nextdocs: vec![],
        };
        let progs = mcd.programmes();
        assert_eq!(progs[0].title, "The A to Z of TV Cooking");
        assert_eq!(progs.len(), 4);
        let pages = mcd.extra_program_pages();
        assert_eq!(pages[0], "/iplayer/episodes/p05jv04g");
        assert_eq!(pages.len(), 13);
        assert_eq!(pages[1], "/iplayer/episodes/b03mzc66");
        assert_eq!(pages[2], "/iplayer/episodes/b08f17c0");
    }

    // #[test]
    fn test_main_category_document() {
        let idoc = IplayerDocument {
            idoc: select::document::Document::from(include_str!("../testhtml/films1.html")),
        };
        let mcd = MainCategoryDocument {
            maindoc: &idoc,
            selectionresults: vec![],
            nextdocs: vec![],
        };
        let np = mcd.next_pages();
        assert_eq!(np.len(), 1);
        assert_eq!(np[0], "films2.html");
    }
}
