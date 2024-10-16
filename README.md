<!--
SPDX-FileCopyrightText: 2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->


# MetaMorph - deploy tool for your deploy tool

Metamorph takes a config file acting as a deployment plan, and runs morph actions on all
hosts in the order specified. All hosts with the same order are massively asynced via the tokio runtime. 

Generate an example config with `metamorph --example > config.yaml`.

```yaml
deploy_sets:
- order: 0
  hosts:
  - devp-stg
  - nixbuild-stg
- order: 1
  hosts:
  - devp-prod
  - nixbuild-prod
```

Use it with e.g. `metamorph -c config.yaml push`.
