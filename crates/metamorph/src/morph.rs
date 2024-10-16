// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use futures::future::join_all;
use futures::Future;
use std::process::Output;
use std::{collections::BTreeMap, ffi::OsStr};
use tokio::{process::Command, task::JoinHandle};

use crate::data::{Config, DeploySet};

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
    //    .args(args)
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

pub async fn foreach_deploy_set<I, S>(config: Config, action: &'static str, args: I)
where
  I: IntoIterator<Item = S> + std::marker::Send + Clone + 'static,
  S: AsRef<OsStr> + 'static,
{
  // jobs has order as key
  let mut jobs: BTreeMap<u64, Vec<JoinHandle<_>>> = BTreeMap::new();

  // vec of encountered orderings (kinda eww tbh)
  let mut order: Vec<u64> = vec![];

  for deploy_set in config.deploy_sets {
    if !jobs.contains_key(&deploy_set.order) {
      order.push(deploy_set.order);
      jobs.insert(deploy_set.order, vec![]);
    }
    jobs
      .get_mut(&deploy_set.order)
      .expect("failed to get deploy_set from jobs map with order {order}")
      .append(&mut foreach_host(action, args.clone(), deploy_set).await)
  }

  order.sort();

  for step in order {
    let commands = join_all(
      jobs
        .get_mut(&step)
        .expect("failed to get order {step} from jobs"),
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
