use crate::{
    bench::{Pallet, Pallets},
    Result,
};
use anyhow::{ensure, Context};
use regex::Regex;
use std::{
    collections::BTreeMap,
    path::Path,
    process::{Command, Output, Stdio},
};

pub struct Node<'a> {
    path: &'a Path,
}

impl<'a> Node<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    pub fn list_pallets(&self) -> Result<Pallets> {
        let mut cmd = self.list_command();
        let output = cmd
            .output()
            .with_context(|| format!("Failed to execute command: {:#?}", cmd))?;
        let stdout = extract_stdout(output).context("Failed to extract stdout")?;
        let re = Regex::new(r#"^(.+?), (.+)"#)?;
        let mut result = BTreeMap::new();
        for line in stdout.lines().skip(1).filter_map(|line| re.captures(line)) {
            let pallet = line
                .get(1)
                .context("Failed to extract pallet name")?
                .as_str();
            let extrinsic = line
                .get(2)
                .context("Failed to extract extrinsic name")?
                .as_str();
            let entry = result.entry(pallet.to_string()).or_insert_with(|| Pallet {
                name: pallet.to_string(),
                extrinsics: Default::default(),
            });
            entry.extrinsics.insert(extrinsic.to_string());
        }
        Ok(Pallets(result))
    }

    pub fn execute_benchmark(&self, pallet: &str, ext: &str) -> Result<()> {
        let mut cmd = self.execute_command(pallet, ext);
        let output = cmd
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .output()
            .with_context(|| format!("Failed to execute command: {:#?}", cmd))?;
        ensure!(
            output.status.success(),
            "Benchmark returned non zero exit status: {}. stderr:\n{}",
            output.status,
            std::str::from_utf8(&output.stderr).unwrap(),
        );
        ensure!(
            output.stdout.is_empty(),
            "Stdout should never return anything (it is directed to /dev/null)"
        );
        Ok(())
    }

    fn basic_command(&self) -> Command {
        let mut cmd = Command::new(self.path);
        cmd.arg("benchmark")
            .arg("--dev")
            .arg("--execution=wasm")
            .arg("--wasm-execution=compiled")
            .arg("--heap-pages=4096");
        cmd
    }

    fn list_command(&self) -> Command {
        let mut cmd = self.basic_command();
        cmd.arg("--list");
        cmd
    }

    fn execute_command(&self, pallet: &str, ext: &str) -> Command {
        let mut cmd = self.basic_command();
        cmd.arg("--pallet")
            .arg(pallet)
            .arg("--extrinsic")
            .arg(ext)
            .arg("--repeat=1");
        cmd
    }
}

fn extract_stdout(output: Output) -> Result<String> {
    ensure!(
        output.status.success(),
        "Command returned non zero status code"
    );
    String::from_utf8(output.stdout).context("Invalid UTF-8 in output")
}
