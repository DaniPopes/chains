#!/bin/sh
set -e

data_dir=data

chains_url=https://chainid.network/chains.json
chains_file=chains.json

mini_chains_url=https://chainid.network/chains_mini.json
mini_chains_file=chains_mini.json

cd $data_dir
wget -N $chains_url $mini_chains_url
zip $(basename $chains_file .json).zip $chains_file
zip $(basename $mini_chains_file .json).zip $mini_chains_file
