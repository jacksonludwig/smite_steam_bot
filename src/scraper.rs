use soup::prelude::*;

pub fn scrape_starter(soup: Soup) -> String {
    let results = soup
        .tag("div")
        .find_all()
        .map(|div| div.name().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    results
}
