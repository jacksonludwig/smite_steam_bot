use scraper::{Html, Selector};

// TODO: return a vector of the items.
pub fn scrape_starter<'a>(soup: &'a Html) -> () {
    let div_selector = Selector::parse("div.pro-build__title").unwrap();

    // TODO: lower loop prob redone so that we just check next sibling's children's children's alts.

    println!("filtering started");
    for item in soup.select(&div_selector) {
        let item_text = item
            .text()
            .filter(|x| x != &" ")
            .take_while(|x| x != &"Final Build")
            .collect::<Vec<_>>();
        println!("{:?}", item_text);
    }
}
