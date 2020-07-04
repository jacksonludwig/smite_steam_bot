mod scraper;

use isahc::prelude::*;
use soup::prelude::*;

fn main() -> Result<(), isahc::Error> {
    let mut webpage = isahc::get("https://smite.guru/builds/ares")?;
    let soup = Soup::new(&webpage.text()?);

    let text = scraper::scrape_starter(soup);
    println!("{}", text);

    Ok(())
}
