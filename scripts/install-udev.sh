#!/bin/sh
set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
install -Dm644 "$project_dir/packaging/udev/99-spenblet-uinput.rules" /etc/udev/rules.d/99-spenblet-uinput.rules
udevadm control --reload-rules
udevadm trigger --name-match=uinput
