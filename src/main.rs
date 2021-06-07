mod bench;
mod cli;
mod node;

use crate::{
    cli::{Command, Opt},
    node::Node,
};
use anyhow::{Context, Result};
use std::path::PathBuf;
use structopt::StructOpt;

fn default_path() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?.canonicalize()?;
    path.push("target/release/substrate");
    Ok(path)
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let path = if let Some(path) = opt.node {
        path
    } else {
        default_path()?
    };
    let node = Node::new(&path);
    let pallets = node.list_pallets().context("Failed to enumerate pallets")?;

    match opt.cmd {
        Command::List { show_ext } => {
            for (key, value) in &pallets.0 {
                if show_ext {
                    println!("{}", value);
                } else {
                    println!("{}", key);
                }
            }
        }
        Command::Bench { pallet, ext } => {
            let pallets = pallets
                .filtered(pallet.as_deref(), ext.as_deref())
                .context("Failed to filter extrinsics")?;
            bench::run(&pallets, &node).context("Failed to run benchmarks")?;
        }
    }

    Ok(())
}
