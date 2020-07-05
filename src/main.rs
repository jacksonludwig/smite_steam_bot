mod retriever;

use isahc::prelude::*;
use soup::prelude::*;

fn main() -> Result<(), isahc::Error> {
    let mut webpage = isahc::get("https://smite.guru/builds/anubis")?;
    let soup = Soup::from_reader(webpage.text()?.as_bytes());

    println!("Scraping started");

    let starter_items = retriever::scrape_starter(&soup.unwrap());
    println!("{:?}", starter_items);

    Ok(())
}
