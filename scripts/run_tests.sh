#!/usr/bin/env bash

set -e

export TESTING=1
features="
  conntrack,
  rt-link,
  rt-addr,
  wireguard,
  nftables,
  nl80211,
  tc,
  inet-diag,
  netdev
"
examples="
  conntrack
  wireguard-setup
  nftables
  nftables-api
  nl80211
  nl80211-raw
  tc-prio
  tcp-rtt
  multicast-simple
  multicast-generic
  multicast-rtnetlink
"

run() {
  echo >&2
  echo ">" "$@" >&2
  command "$@"
}

cargo() {
  run cargo "$@" --features="$(echo $features | tr -d " ")"
}

matches() {
  if ! rg --passthru -- "$1"; then
    echo
    echo "Error: Pattern didn't match. Expected: $1"
    exit 1
  fi
}

run_vm() {
  if ! type -P nix &>/dev/null; then
    echo "Skipping vm tests: 'nix' command not found"
    return
  fi

  if test -z "$vm_runner"; then
    local out="$(
      run nix build \
        --print-out-paths --no-link \
        -f ./scripts/vm_tests.nix \
        --argstr bins "$(echo $examples)" \
        --argstr bin_dir "target/debug/examples" \
        driver
    )"
    vm_runner="$out/bin/nixos-test-driver"
  fi

  # To debug inside the vm run `$vm_runner --interactive`, type
  # `machine.start()`, wait until it boots, and ssh root@vsock/7502{0,1,2,...}
  run "$vm_runner" # [--interactive]
}

cargo check -p netlink-bindings --all-features

if ! ip link show wg0 >/dev/null; then
  # Create "wg0" interface for doctests in readme
  ip link add dev wg0 type wireguard
fi

cargo test

for runtime in "" tokio smol; do
  cargo run --example=extack |
    matches 'Attribute failed policy validation: attribute "Ifname" in "LinkAttrs": PolicyTypeAttrs \{ MaxLength: 15, Type: 11 \}'

  for example in $examples; do
    cargo run --example="$example" --features="$runtime"
  done

  # Run the same examples in a VM with a bunch of different kernel versions
  run_vm
done
