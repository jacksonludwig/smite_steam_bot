use isahc::prelude::*;
use soup::prelude::*;

const STARTER_ITEM_AMT_DEFAULT: usize = 7;
const ENDING_ITEM_AMT_DEFAULT: usize = 5;

const STARTER_BUILD_INDEX: usize = 6;
const ENDING_BUILD_INDEX: usize = 16;
const ENDING_RELIC_OFFSET: usize = 6;

const BUILD_DIV_TAG: &str = "data-v-c00756d0";

fn scrape_build(soup: &Soup, build_index: usize) -> Vec<String> {
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

pub struct Builder {
    soup: Soup,
}

impl Builder {
    pub fn new(link: &str) -> Result<Builder, isahc::Error> {
        let mut webpage = isahc::get(link)?;
        let soup = Soup::from_reader(webpage.text()?.as_bytes())?;
        let builder = Builder { soup };
        Ok(builder)
    }

    pub fn scrape_beg_and_end(&self) -> Vec<Vec<String>> {
        let starter = scrape_build(&self.soup, STARTER_BUILD_INDEX);

        let end_index = ENDING_BUILD_INDEX - (STARTER_ITEM_AMT_DEFAULT - starter.len());
        let mut ending = scrape_build(&self.soup, end_index);

        let end_relic_index =
            end_index + ENDING_RELIC_OFFSET + ending.len() - ENDING_ITEM_AMT_DEFAULT;
        let mut ending_relics = scrape_build(&self.soup, end_relic_index);

        ending.append(&mut ending_relics);

        vec![starter, ending]
    }
}
