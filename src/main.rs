mod retriever;

use retriever::Builder;

fn main() -> Result<(), isahc::Error> {
    let builder = Builder::new("https://smite.guru/builds/achilles").unwrap();
    let items = builder.scrape_beg_and_end();

    println!("{:?}", items[0]);
    println!("{:?}", items[1]);

    Ok(())
}
