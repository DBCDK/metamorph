/*
 * SPDX-FileCopyrightText: 2024 Christina Sørensen
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
  pub deploy_sets: Vec<DeploySet>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeploySet {
  pub order: u64,
  pub hosts: Vec<String>,
  /// Whether or not to wait for user input before deploying set
  #[serde(default)]
  pub confirm: bool,
}

impl Config {
  fn parse(content: &str) -> Self {
    serde_norway::from_str(content).expect("Failed to parse config")
  }
  pub fn load(path: &str) -> Self {
    Self::parse(&std::fs::read_to_string(path).expect("Failed to open {path}"))
  }
  pub fn output_example_config() {
    println!(
      "{}",
      serde_norway::to_string(&Self::generate_example())
        .expect("Failed to turn example config into string")
    );
  }
  pub fn generate_example() -> Self {
    Self {
      deploy_sets: vec![
        DeploySet {
          order: 0,
          hosts: vec!["devp-stg".into(), "nixbuild-stg".into()],
          ..Default::default()
        },
        DeploySet {
          order: 1,
          hosts: vec!["devp-prod".into(), "nixbuild-prod".into()],
          confirm: true,
        },
      ],
    }
  }
}
