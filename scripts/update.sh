#!/usr/bin/env bash
set -e

chains_url=https://chainid.network/chains.json
chains_file=data/chains.json

curl $chains_url > $chains_file
