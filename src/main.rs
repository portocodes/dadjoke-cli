extern crate clap;
use clap::{App, Arg};

extern crate reqwest;
use reqwest::Client;

fn main() {
    let matches = App::new("dadjokes")
        .version("1.0")
        .about("Tells dad jokes!")
        .author("Ricardo Mendes")
        .arg(
            Arg::with_name("search")
                .short("s")
                .long("search")
                .value_name("SEARCH")
                .help("Searches for a specific term")
                .takes_value(true),
        )
        .get_matches();

    let client = Client::new();
    let piada = match matches.value_of("search") {
        Some(search) => client
            .get("https://icanhazdadjoke.com/search")
            .header(reqwest::header::ACCEPT, "text/plain")
            .query(&[("term", search), ("limit", "1")]),
        None => client
            .get("https://icanhazdadjoke.com/")
            .header(reqwest::header::ACCEPT, "text/plain"),
    };

    println!(
        "Olá Porto Codes!\n A piada de hoje é: {:?}",
        piada.send().unwrap().text().unwrap()
    );
}
