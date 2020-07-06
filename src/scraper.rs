use super::gods;
use soup::prelude::*;

const STARTER_ITEM_AMT_DEFAULT: isize = 7;
const ENDING_ITEM_AMT_DEFAULT: isize = 5;

const STARTER_BUILD_INDEX: isize = 6;
const ENDING_BUILD_INDEX: isize = 16;
const ENDING_RELIC_OFFSET: isize = 6;

const BUILD_DIV_TAG: &str = "data-v-c00756d0";

fn scrape_build(soup: &Soup, build_index: isize) -> Vec<String> {
    let pro_div = soup
        .tag("div")
        .attr(BUILD_DIV_TAG, "")
        .find_all()
        .take_while(|e| &e.text() != &"Leveling Order")
        .collect::<Vec<_>>();

    pro_div[build_index as usize]
        .tag("img")
        .find_all()
        .map(|e| e.get("alt").unwrap())
        .collect::<Vec<_>>()
}

struct Builder {
    soup: Soup,
}

impl Builder {
    fn new(html: &str) -> Builder {
        let soup = Soup::new(html);
        Builder { soup }
    }

    fn scrape_beg_and_end(&self) -> Vec<Vec<String>> {
        let starter = scrape_build(&self.soup, STARTER_BUILD_INDEX);

        let end_index = ENDING_BUILD_INDEX - (STARTER_ITEM_AMT_DEFAULT - (starter.len() as isize));
        let mut ending = scrape_build(&self.soup, end_index);

        let end_relic_index =
            end_index + ENDING_RELIC_OFFSET + (ending.len() as isize) - ENDING_ITEM_AMT_DEFAULT;
        let mut ending_relics = scrape_build(&self.soup, end_relic_index);

        ending.append(&mut ending_relics);

        vec![starter, ending]
    }
}

pub fn get_god_build(god: &str) -> Vec<Vec<String>> {
    let html = gods::read_html_from_file(god);
    let builder = Builder::new(&html);
    let items = builder.scrape_beg_and_end();
    items
}
