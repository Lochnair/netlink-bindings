#!/bin/sh

set -e

force=
changed=

if [ $# -ne 0 ]; then
  force=1
fi

if ! test -f netlink-bindings/src/lib.rs; then
  echo "Should be run from project root"
  exit 1
fi

codegen="$(cargo run --bin codegen --config 'target."cfg(true)".runner="echo"')"

for subsys in netlink-bindings/src/*/; do
  subsys_name="$(basename -- "$subsys")"
  yaml="$subsys/$subsys_name.yaml"
  overrides_yaml="$subsys/$subsys_name.overrides.yaml"
  mod="$subsys/mod.rs"
  dump="$subsys/$subsys_name.md"

  if [ -z "$force" ] &&
    [ -e "$mod" ] &&
    [ "$mod" -nt "$codegen" ] &&
    [ "$mod" -nt "$yaml" ] &&
    [ ! -e "$overrides_yaml" -o "$mod" -nt "$overrides_yaml" ]; then
    echo Skipping "$subsys_name"
    continue
  fi

  args=""
  case "$subsys_name" in
  # For most subsystems, dump files get too large to be useful
  nlctrl | wireguard)
    args="$args --dump $dump"
    ;;
  esac

  echo Generating "$subsys_name"
  command "$codegen" -d "$subsys" $args

  changed=1
done

gen="./reverse-lookup/src/generated.rs"

if [ -n "$force" ] || [ ! -e "$gen" ] || [ -n "$changed" ]; then
  echo Generating reverse-lookup
  command "$codegen" -d ./netlink-bindings/src --reverse-lookup "$gen"
else
  echo Skipping reverse-lookup
fi
