mod retriever;

use isahc::prelude::*;
use scraper::Html;

fn main() -> Result<(), isahc::Error> {
    let mut webpage = isahc::get("https://smite.guru/builds/ares")?;
    let soup = Html::parse_document(&webpage.text()?);

    retriever::scrape_starter(&soup);

    Ok(())
}
