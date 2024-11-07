# SPDX-FileCopyrightText: 2024 Christina Sørensen
#
# SPDX-License-Identifier: EUPL-1.2

default: 
  just --list

update-deps:
  cargo hakari generate

generate-diagram:
  dot -Tpng diagram.dot > diagram.png && icat diagram.png
