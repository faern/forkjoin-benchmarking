#!/bin/bash

sudo apt-get install -y git htop vim gcc gnuplot
#curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
#multirust default nightly-2016-02-02

wget https://static.rust-lang.org/dist/2016-02-02/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
tar xf rust-nightly-x86_64-unknown-linux-gnu.tar.gz
sudo ./rust-nightly-x86_64-unknown-linux-gnu/install.sh

git clone https://github.com/faern/forkjoin-benchmarking
cd forkjoin-benchmarking

cargo build --release --features linux-affinity

rm -rf .criterion/ && cargo run --release --features linux-affinity -- fib qsort mergesort sumtree_unbalanced nqueens_reduce nqueens_search --threads 1 2 4 8 12 16 20 24 28 32 --fib 42 --sort 10000000 --sumtree 23 --nqueens 12 -s 20 | tee output.txt && mv output.txt .criterion/

tar -czf $HOME/criterion-data.tar.gz .criterion/*

