#!/bin/sh
set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
adb forward tcp:27183 tcp:27183
exec "$project_dir/linux/target/release/spenblet-daemon"
