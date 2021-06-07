# benchbencher

Yo dawg heard you like benchmarks. Now you can benchmark while you benchmark.

![Demo of the Application](https://user-images.githubusercontent.com/2580396/121548672-7f0ee400-ca0d-11eb-81aa-2ebfab433118.mp4)

## Installation

```shell
cargo install --git https://github.com/athei/benchbencher
```

## Usage

```
benchbencher 0.1.0
Benchmark the resource usage of your substrate runtime benchmarks

USAGE:
    benchbencher [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --node <node>    Path to your node which was compiled with the `runtime-benchmarks` feature. If left out the
                         following path is used: <PWD>/target/release/substrate

SUBCOMMANDS:
    bench    Run the specified benchmarks
    help     Prints this message or the help of the given subcommand(s)
    list     List available benchmarks
```

## What is this?

This application simply runs the benchmarks contained in a substrate node (by executing the node binary) and measures the wall clock time each benchmark needed to execute. Note that this includes the setup closure of the benchmark which does **not** contribute to the benchmarks results but can have a substantial overhead.

This is handy to identify find benchmarks which have a disproportional long running time and need some optimization. This is mostly relevant for the contracts pallet that alone has over 100 benchmarks with some of them having multi second setup running time.

Please note that this is merely useful as a means to compare the relative running time between benchmarks. The absolute number has no meaning because it includes the startup time of the benchmarking infrastructure itself.
