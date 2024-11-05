// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2
#![allow(clippy::async_yields_async)]

use futures::future::join_all;
use futures::Future;
use std::process::Output;
use std::{
  collections::{BTreeMap, BTreeSet},
  ffi::OsStr,
};
use tokio::{process::Command, task::JoinHandle};

use crate::data::{Config, DeploySet};

/// A u64 ordering of a deployment_set
type Order = u64;

const MORPH_COMMAND: &str = "morph";
const DRY_RUN_COMMAND: &str = "echo";

#[inline]
fn morph_command() -> &'static str {
  match crate::DRY_RUN
    .get()
    .expect("failed to get DRY_RUN in morph_command, DRY_RUN may not be initialized")
  {
    true => DRY_RUN_COMMAND,
    false => MORPH_COMMAND,
  }
}

#[inline]
async fn morph_action<I, S>(
  command: &str,
  deployment: String,
  args: I,
) -> impl Future<Output = tokio::io::Result<Output>>
where
  I: IntoIterator<Item = S> + std::marker::Send + Clone,
  S: AsRef<OsStr> + 'static,
{
  log::trace!("{command} {deployment}");
  Command::new(morph_command())
    .arg(command)
    .arg(deployment)
    .args(args)
    .output()
}

async fn foreach_host<I, S>(
  action: &'static str,
  args: I,
  deploy_set: DeploySet,
) -> Vec<JoinHandle<impl Future<Output = impl Future<Output = Result<Output, std::io::Error>>>>>
where
  I: IntoIterator<Item = S> + std::marker::Send + Clone + 'static,
  S: AsRef<OsStr> + 'static,
{
  deploy_set
    .hosts
    .into_iter()
    .map(|host| {
      let cloned_args = args.clone();
      tokio::spawn(async move {
        log::trace!("{action} to {host}");
        morph_action(action, host, cloned_args)
      })
    })
    .collect()
}

const CONFIRMATION_YES: &str = "y";
const CONFIRMATION_NO: &str = "n";

/// Pauses execution, awaiting user confirmation
async fn get_confirmation() -> bool {
  use futures::StreamExt;
  use tokio::io::stdin;
  use tokio_util::codec::{FramedRead, LinesCodec};

  let mut reader = FramedRead::new(stdin(), LinesCodec::new());

  println!("Continue deploying (Y/n)");

  let mut input_buffer = reader.next().await.transpose().unwrap().unwrap();

  // Normalize input
  input_buffer = input_buffer.to_lowercase();

  // Remove newline/return from input
  if let Some('\n') = input_buffer.chars().next_back() {
    input_buffer.pop();
  }
  if let Some('\r') = input_buffer.chars().next_back() {
    input_buffer.pop();
  }

  match input_buffer.as_str() {
    CONFIRMATION_YES => true,
    CONFIRMATION_NO => false,
    _ => {
      println!("Please answer before continuing");
      Box::pin(get_confirmation()).await
    }
  }
}

pub async fn foreach_deploy_set<I, S>(config: Config, action: &'static str, args: I)
where
  I: IntoIterator<Item = S> + std::marker::Send + Clone + 'static,
  S: AsRef<OsStr> + 'static,
{
  let mut jobs: BTreeMap<Order, Vec<JoinHandle<_>>> = BTreeMap::new();

  let mut seen_orderings: BTreeSet<Order> = BTreeSet::new();

  for deploy_set in config.deploy_sets {
    // Check if deploy set wants confirmation to proceed
    if deploy_set.confirm {
      match get_confirmation().await {
        true => (),
        false => std::process::exit(0),
      }
    }

    jobs.entry(deploy_set.order).or_insert_with(|| {
      seen_orderings.insert(deploy_set.order);
      vec![]
    });

    jobs
      .get_mut(&deploy_set.order)
      .expect("failed to get deploy_set from jobs map with order {order}")
      .append(&mut foreach_host(action, args.clone(), deploy_set).await)
  }

  for order in seen_orderings {
    log::trace!("Running ordeing order: {order}");
    let commands = join_all(
      jobs
        .get_mut(&order)
        .expect("failed to get order {order} from jobs"),
    )
    .await
    .into_iter()
    .map(|res| async {
      let res = res.unwrap().await.await.unwrap();
      println!("{res:?}");
      res
    })
    .collect::<Vec<_>>();
    join_all(commands).await;
  }
}

// Yes... these shouldn't all be statics...
