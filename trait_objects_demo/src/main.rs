use std::cell::RefCell;
use trait_objects_demo::{WebPage, Html, Js, Media};

fn main() {

    let mut wp = WebPage::new();
    load_web_page_elements(&mut wp);
    download_and_render_web_page(&wp);
}

fn load_web_page_elements(web_page: &mut WebPage) {

    for i in 1..7 {
        web_page.elements.push(Box::new(Html {
            contents: format!("<h{}>Hello</h{}>", i, i)
        }));
    }

    web_page.elements.push(Box::new(Html {
        contents: String::from("<p>Lorem ipsum</p>")
    }));

    web_page.elements.push(Box::new(Js {
        scripts: String::from("console.log('hello world');")
    }));

    web_page.elements.push(Box::new(Media {
        images: RefCell::new(vec!(
            "/1.jpg".to_string(), 
            "/2.jpg".to_string()))
    }))
}

fn download_and_render_web_page(web_page: &WebPage) {
    // Download elements for the webpage
    for element in &web_page.elements {
        element.download().unwrap();
    }

    // Render elements for the webpage
    for element in &web_page.elements {
        element.render();
    }
}
