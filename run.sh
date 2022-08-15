#!/bin/bash

packfolder src/ui target/assets.rc -binary
if [ "$1" = "--release" ]; then
  cargo build --release
  makensis installer.nsi
else
  cargo run
fi