#! /usr/bin/env bash

cargo changelog create-release custom "$NEW_VERSION"
cargo changelog generate-changelog --allow-diry
