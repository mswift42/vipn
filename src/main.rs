extern crate select;
use select::predicate::{Predicate, Class, Name};

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
            .unwrap();
        if path.starts_with("http://www.bbc.co.uk") {
            path.to_string()
        } else {
            "http://www.bbc.co.uk".to_string() + path
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
    fn new(inode: &'a IplayerNode) -> IplayerSelection<'a> {
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
                programme: Some(Programme::new(inode)),
                extra_prog_page: None
            }
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
    fn new(inode: &'a IplayerNode) -> Programme<'a> {
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

    #[test]
    fn test_document() {
        let doc =  IplayerDocument::from(include_str!("../testhtml/food1.html"));
        assert_eq!(&doc.find(Name("h1")).next().unwrap().text(), " Food  - A-Z ");
        let dn = doc.find(Class("list-item-inner")).next().unwrap();
        let inode  = IplayerNode { node: dn };
        assert_eq!(inode.find_title(), "The A to Z of TV Cooking");
        let prog = Programme::new(&inode);
        println!("{:?}", prog);
    }
}
