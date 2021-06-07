use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "benchbencher",
    about = "Benchmark the resource usage of your substrate runtime benchmarks"
)]
pub struct Opt {
    /// Path to your node which was compiled with the `runtime-benchmarks` feature.
    /// If left out the following path is used: <PWD>/target/release/substrate
    #[structopt(long, parse(from_os_str))]
    pub node: Option<PathBuf>,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// Run the specified benchmarks
    Bench {
        /// The pallet whose benchmarks are benchmarked. Leave out to benchmark all.
        #[structopt(short, long)]
        pallet: Option<String>,
        /// Some extrinsic of the specified `pallet` which should be benchmarked.
        /// This is mainly useful for development where one is working on a single benchmark.
        /// Only allowed to be set when `pallet` is also set.
        #[structopt(short, long)]
        ext: Option<String>,
    },
    /// List available benchmarks.
    List {
        /// List extrinsics. Otherwise only the pallets are shown.
        #[structopt(long)]
        show_ext: bool,
    },
}
