// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use clap::{arg, command, crate_authors, Arg, ArgAction, ArgMatches, Command};

pub fn build_cli() -> Command {
  command!()
    .author(crate_authors!("\n"))
    .arg(arg!([host_set] "A set of hosts").required(false))
    .arg(arg!(-c --config [config] "Specify config file"))
    .arg(arg!(--example "Generate example config file"))
    .arg(arg!(-v --verbose ... "Verbosity level."))
    .arg(arg!(--passcmd [passcmd] "Set password manager command"))
    .subcommand(
      Command::new("push")
        .about("push out changes to hostgroup")
        .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
    )
    .arg(
      Arg::new("dryrun")
        .long("dry-run")
        .action(ArgAction::SetTrue)
        .help("Replace all morph commands with echo"),
    )
}

pub fn set_verbosity(matches: &ArgMatches) {
  use std::env;
  match matches
    .get_one::<u8>("verbose")
    .expect("Counts aren't defaulted")
  {
    0 => env::set_var("RUST_LOG", "error"),
    1 => env::set_var("RUST_LOG", "warn"),
    2 => env::set_var("RUST_LOG", "info"),
    3 => env::set_var("RUST_LOG", "debug"),
    4 => env::set_var("RUST_LOG", "trace"),
    _ => {
      log::trace!("More than four -v flags don't increase log level.");
      env::set_var("RUST_LOG", "trace")
    }
  }
}
