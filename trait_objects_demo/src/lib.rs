use std::result::Result;

pub struct WebPage {
    pub elements: Vec<Box<dyn WebPageElement>>,
}

impl WebPage {
    pub fn new() -> WebPage {
        WebPage {
            elements: vec!(),
        }
    }
}

pub trait WebPageElement {
    fn download(&self) -> Result<&str, String>;
    fn render(&self) -> ();
}

pub struct Html {
    pub contents: String
}

impl WebPageElement for Html {
    fn download(&self) -> Result<&str, String> {
        Ok(&self.contents)
    }

    fn render(&self) { 
        println!("Html:\t{}", self.contents);
    }
}

pub struct Js {
    pub scripts: String
}

impl WebPageElement for Js {
    fn download(&self) -> Result<&str, String> { 
        Ok(&self.scripts)
    }

    fn render(&self) {
        println!("Js:\t{}", self.scripts);
    }
}