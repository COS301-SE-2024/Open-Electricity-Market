#!/bin/bash
cd simulation
cargo build
cd ..
cd market-platfrom
cargo build
cd ..
cd agent
cargo build
cd ..
killbg() {
	for p in "{$pids[@]}" ; do
		echo "&p";
		kill "&p";
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
