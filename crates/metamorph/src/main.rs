// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2
//
// Deploy trivial morph hosts (trivial meaning without special build instructions)

mod cli;
mod data;
mod morph;

use std::{io::Error, sync::OnceLock};

static DRY_RUN: OnceLock<bool> = OnceLock::new();
static IMPURE: OnceLock<bool> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Error> {
  let matches = cli::build_cli().get_matches();

  fast_log::init(fast_log::Config::new().console().chan_len(Some(100000))).unwrap();

  cli::set_verbosity(&matches);

  if matches.get_flag("example") {
    data::Config::output_example_config();
    return Ok(());
  }

  let config;
  if let Some(config_file) = matches.get_one::<String>("config") {
    config = data::Config::load(config_file);
  } else {
    config = data::Config::generate_example();
  }

  if matches.get_flag("dryrun") {
    DRY_RUN
      .set(true)
      .expect("failed to set DRY_RUN OnceLock to true");
  } else {
    DRY_RUN
      .set(false)
      .expect("failed to set DRY_RUN OnceLock to false");
  }

  if matches.get_flag("impure") {
    IMPURE
      .set(true)
      .expect("failed to set IMPURe OnceLock to true");
  } else {
    IMPURE
      .set(false)
      .expect("failed to set IMPURE OnceLock to false");
  }

  let mut args: Vec<String> = vec![];

  if matches.get_flag("keepresults") {
    args.push("--keep-results".into());
  }

  if let Some(passcmd) = matches.get_one::<String>("passcmd") {
    args.push("--passcmd".into());
    args.push(passcmd.clone());
  } else if let Some(_passfile) = matches.get_one::<String>("passfile") {
    // TODO: implement this in morph
    todo!()
  }

  match matches.subcommand() {
    Some(("push", _)) => morph::foreach_deploy_set(config, "push", args).await,
    Some(("boot", sub_matches)) => {
      if sub_matches.get_flag("reboot") {
        args.push("--reboot".into());
      };
      morph::foreach_deploy_set(config, "boot", args).await;
    }
    Some(("switch", _)) => morph::foreach_deploy_set(config, "switch", args).await,
    _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
  }

  Ok(())
}
