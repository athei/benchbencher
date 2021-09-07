use crate::{node::Node, Result};
use anyhow::{ensure, Context};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryFrom,
    fmt,
    time::{Duration, Instant},
};

pub struct Pallets(pub BTreeMap<String, Pallet>);

#[derive(Clone)]
pub struct Pallet {
    pub name: String,
    pub extrinsics: BTreeSet<String>,
}

struct BenchmarkedExtrinsic {
    name: String,
    elapsed: Duration,
}

impl Pallets {
    pub fn filtered(&self, pallet: Option<&str>, ext: Option<&str>) -> Result<Vec<Pallet>> {
        let iter = self.0.values().cloned();
        let mut pallets: Vec<Pallet> = if let Some(pallet) = pallet {
            iter.filter(|val| val.name == pallet).collect()
        } else {
            iter.collect()
        };
        ensure!(!pallets.is_empty(), "Pallet not found");

        if let Some(ext) = ext {
            ensure!(
                pallets.len() == 1,
                "An exernality can only be supplied when also filtering the pallets"
            );
            ensure!(pallets[0].extrinsics.contains(ext), "Extrinsic not found");
            let mut extrinsics = BTreeSet::new();
            extrinsics.insert(ext.to_string());
            pallets[0].extrinsics = extrinsics;
        }

        Ok(pallets)
    }
}

impl fmt::Display for Pallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {{", self.name)?;
        for ex in &self.extrinsics {
            writeln!(f, "    {},", ex)?;
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for BenchmarkedExtrinsic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<80}: {}", self.name, format_duration(self.elapsed))
    }
}

pub fn run(pallets: &[Pallet], node: &Node) -> Result<()> {
    let num: usize = pallets.iter().map(|p| p.extrinsics.len()).sum();
    let mut results = Vec::with_capacity(num);
    let pallet_progress = ProgressBar::new(u64::try_from(num)?);
    pallet_progress.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.cyan.bold} [{bar:66}] {pos:>2}/{len:2}: {msg}")
            .progress_chars("=> "),
    );
    pallet_progress.set_prefix("Benchmarking");

    for pallet in pallets {
        for ext in &pallet.extrinsics {
            let name = format!("{}/{}", pallet.name, ext);
            pallet_progress.set_message(name.clone());
            // for whaterever reason the message is only updated when a line is printed
            pallet_progress.println("");
            let start = Instant::now();
            node.execute_benchmark(&pallet.name, ext)
                .with_context(|| format!("Failed to execute: {}", name))?;
            let elapsed = start.elapsed();
            let result = BenchmarkedExtrinsic { name, elapsed };
            pallet_progress.inc(1);
            pallet_progress.println(format!("{}", result));
            results.push(result);
        }
    }
    pallet_progress.finish_and_clear();

    results.sort_unstable_by_key(|b| b.elapsed);
    results.reverse();
    let sum = format_duration(results.iter().map(|b| b.elapsed).sum());
    println!("\n{}", style("Sorted Results").cyan().bold());
    for result in results {
        println!("{}", result);
    }
    println!("{:<80}: {}", style("Sum").yellow(), style(sum).yellow());

    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let clipped = Duration::from_millis(duration.as_millis() as u64);
    humantime::format_duration(clipped).to_string()
}
