// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use clap::{arg, command, crate_authors, Arg, ArgAction, ArgMatches, Command};

pub fn build_cli() -> Command {
  command!()
    .author(crate_authors!("\n"))
    // Local top-level args
    .arg(arg!([host_set] "A set of hosts").required(false))
    .arg(arg!(--example "Generate example config file"))
    // Global top-level args
    .arg(arg!(-c --config [config] "Specify config file").global(true))
    .arg(arg!(-v --verbose ... "Verbosity level.").global(true))
    .arg(arg!(--impure "Don't clear host environemnt variables from morph invocatins").global(true))
    .arg(Arg::new("keepresults").global(true).long("keep-results").action(ArgAction::SetTrue).help("Keep latest build in .gcroots to prevent it from being garbage collected"))
    .arg(
      arg!(--passcmd [passcmd] "Set password manager command")
        .global(true)
        .conflicts_with("passfile"),
    )
    .arg(
      arg!(--passfile [passfile] "Sets a file to read password from (it's strongly preffered to use passcmd instead of this)")
        .global(true)
        .conflicts_with("passcmd"),
    )
    .arg(
      Arg::new("dryrun")
        .long("dry-run")
        .action(ArgAction::SetTrue)
        .global(true)
        .help("Replace all morph commands with echo"),
    )
    // Subcommands
    .subcommand(
      Command::new("push")
        .about("Push closures to host")
    )
    .subcommand(
      Command::new("switch")
        .about("Push closures to host")
    )
    .subcommand(
      Command::new("boot")
        .about("Deploys boot entry without switching")
        .arg(arg!(--reboot "Reboots hosts after creating new boot entry").action(ArgAction::SetTrue)),
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
