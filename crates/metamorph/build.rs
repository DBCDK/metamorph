// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use clap::ValueEnum;
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use std::env;
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

include!("src/cli.rs");

const BIN_NAME: &str = "metamorph";

fn main() -> Result<(), Error> {
  let real_outdir = match env::var_os("OUT_DIR") {
    None => return Ok(()),
    Some(outdir) => outdir,
  };

  let outdir = match env::var_os("MAN_OUT") {
    None => real_outdir,
    Some(outdir) => outdir,
  };

  let mut cmd = build_cli();
  for &shell in Shell::value_variants() {
    // HACK: this is gross :(
    std::process::Command::new("mkdir")
      .arg("man")
      .output()
      .expect("failed to make man directory");

    generate_to(shell, &mut cmd, BIN_NAME, &outdir)?;
  }

  let file = PathBuf::from(&outdir).join(format!("{BIN_NAME}.1"));
  let mut file = File::create(file)?;

  Man::new(cmd).render(&mut file)?;

  println!("cargo:warning=completion file is generated: {outdir:?}");

  Ok(())
}
