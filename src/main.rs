use anyhow::{anyhow, Result};
use clap::{App, Arg, SubCommand};

use async_std::task;

mod zettel;

use crate::zettel::Zettel;

async fn new_zettel(name: &str, tags: Vec<String>) -> Result<()> {
    let zettel = Zettel::new(name, tags);
    let pth = zettel.path_buf();
    zettel.render_to_file(pth).await?;
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
        .get_matches();

    match matches.subcommand() {
        ("new", Some(new_matches)) => {
            let name = new_matches.value_of("name").expect("Unable to read name");
            let tags = match matches.values_of("tags") {
                Some(vals) => vals.map(String::from).collect(),
                None => vec![],
            };
            task::block_on(new_zettel(name, tags)).expect("Unable to spawn run task");
            Ok(())
        }
        (&_, _matches) => Err(anyhow!("Unrecognized subcommand, try 'help'")),
    }
}
