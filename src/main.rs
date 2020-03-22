use anyhow::{anyhow, Result};
use clap::{App, Arg, SubCommand};

use async_std::task;

mod config;
mod zettel;

use crate::config::Config;
use crate::zettel::Zettel;

async fn new_zettel(name: &str, tags: Vec<String>) -> Result<()> {
    let zettel = Zettel::new(name, tags);
    zettel.render_to_file().await?;
    Ok(())
}

async fn init_zettelkasten(path: &str) -> Result<()> {
    Config::init(path)?;
    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("ztl")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("new")
                .about("create new zettel")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("name")
                        .takes_value(true)
                        .value_name("ZETTEL_NAME")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("tags")
                        .long("tag")
                        .short("t")
                        .takes_value(true)
                        .multiple(true)
                        .value_name("TAG"),
                ),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("initialize a new zettelkasten")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("root_path")
                        .takes_value(true)
                        .value_name("ZETTELKASTEN_ROOT")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(init_matches)) => {
            let path = init_matches
                .value_of("root_path")
                .expect("unable to read root path");
            task::block_on(init_zettelkasten(path))
                .expect("Unable to spawn 'init_zettelkasten' task");
            Ok(())
        }
        ("new", Some(new_matches)) => {
            let name = new_matches.value_of("name").expect("Unable to read name");
            let tags = match matches.values_of("tags") {
                Some(vals) => vals.map(String::from).collect(),
                None => vec![],
            };
            task::block_on(new_zettel(name, tags)).expect("Unable to spawn 'new_zettel' task");
            Ok(())
        }
        (&_, _matches) => Err(anyhow!("Unrecognized subcommand, try 'help'")),
    }
}
