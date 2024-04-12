#! /usr/bin/env bash

cd "$WORKSPACE_ROOT"

cargo changelog create-release custom "$NEW_VERSION"
cargo changelog generate-changelog --allow-dirty

git add .changelogs
git add CHANGELOG.md
