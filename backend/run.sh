#!/bin/bash
cd simulation || exit
cargo build
cd .. || exit
cd market-platfrom || exit
cargo build
cd .. || exit
cd agent || exit
cargo build
cd .. || exit
killbg() {
	for p in ${$pids[@]} ; do
		echo & p;
		kill & p;
	done
}
trap killbg EXIT
pids=()
./simulation/target/debug/simulation &
pids+=($!)
./market-platfrom/target/debug/market-platfrom &
pids+=($!)
./agent/target/debug/agent consume 1118 &
pids+=($!)
./agent/target/debug/agent produce 2000
