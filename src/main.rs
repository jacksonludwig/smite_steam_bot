mod gods;
mod scraper;

fn main() -> Result<(), isahc::Error> {
    let items = scraper::get_god_build("khepri");
    println!("{:?}", items[0]);
    println!("{:?}", items[1]);

    Ok(())
}
