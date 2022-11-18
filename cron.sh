#! /bin/bash
set -e

cargo build --release && echo "*/15 * * * * $PWD/target/release/local-money-fiat-price-aggregator >> /var/log/local-price.log 2>&1"