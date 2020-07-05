use soup::prelude::*;

pub fn scrape_starter(soup: &Soup) -> Vec<String> {
    let pro_div = soup
        .tag("div")
        .attr("data-v-c00756d0", "")
        .find_all()
        .take_while(|e| &e.text() != &"Final Build")
        .collect::<Vec<_>>();

    pro_div[6]
        .tag("img")
        .find_all()
        .map(|e| e.get("alt").unwrap())
        .collect::<Vec<_>>()
}
