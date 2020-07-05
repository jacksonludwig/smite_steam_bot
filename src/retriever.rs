use soup::prelude::*;

const BUILD_DIV_TAG: &str = "data-v-c00756d0";
const STARTER_BUILD_INDEX: usize = 6;
const ENDING_BUILD_INDEX: usize = 16;

pub fn scrape_build(soup: &Soup, build_index: usize) -> Vec<String> {
    let pro_div = soup
        .tag("div")
        .attr(BUILD_DIV_TAG, "")
        .find_all()
        .take_while(|e| &e.text() != &"Leveling Order")
        .collect::<Vec<_>>();

    pro_div[build_index]
        .tag("img")
        .find_all()
        .map(|e| e.get("alt").unwrap())
        .collect::<Vec<_>>()
}

pub fn scrape_starter(soup: &Soup) -> Vec<String> {
    scrape_build(soup, STARTER_BUILD_INDEX)
}

pub fn scrape_ending(soup: &Soup) -> Vec<String> {
    scrape_build(soup, ENDING_BUILD_INDEX)
}
