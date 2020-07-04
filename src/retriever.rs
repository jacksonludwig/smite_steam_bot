use scraper::{Html, Selector};

pub fn scrape_starter<'a>(soup: &'a Html) -> () {
    let div_selector = Selector::parse(".pro-build").unwrap();

    println!("filtering started");
    for item in soup.select(&div_selector) {
        let item_text = item.text().filter(|x| x != &" ").collect::<Vec<_>>();
        println!("{:?}", item_text);
    }
}
