mod api;
mod cli;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;
#[macro_use]
extern crate clap;
extern crate serde;

use clap::{App, Arg};
use repositories::pokemon::{Repository ,InMemoryRepository, SqliteRepository};
use std::sync::Arc;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("cli").long("cli").help("Runs in CLI mode"))
        .arg(Arg::with_name("sqlite").long("sqlite").value_name("databse.sqlite"))
        .get_matches();

    let repo = build_repo(matches.value_of("sqlite"));

    match matches.occurrences_of("cli") {
        0 => api::serve("localhost:8080", repo),
        _ => cli::run(repo),
    }
}

fn build_repo(sqlite_value: Option<&str>) -> Arc<dyn Repository> {
    if let Some(path) = sqlite_value {
        match SqliteRepository::try_new(path) {
            Ok(repo) => return Arc::new(repo),
            _ => panic!("Error while creating sqlite repo"),
        }
    }

    Arc::new(InMemoryRepository::new())
}
