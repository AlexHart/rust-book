use std::cell::RefCell;
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

pub struct Media {
    pub images: RefCell<Vec<String>>
}

impl WebPageElement for Media {
    fn render(&self) {
        let images = self.images.borrow();
        for image in &*images {
            println!("Media:\t{}", image);
        }
    }
    
    fn download(&self) -> std::result::Result<&str, std::string::String> { 
        let mut images = self.images.borrow_mut();
        images.push("/images/logo.jpg".to_string());
        images.push("/images/bg.jpg".to_string());
        Ok("done")
    }
}