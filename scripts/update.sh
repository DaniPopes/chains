#!/bin/sh
set -e

chains_url=https://chainid.network/chains.json
chains_file=data/chains.json

curl $chains_url > $chains_file
cd $(dirname $chains_file)
zip $(basename $chains_file .json).zip $(basename $chains_file)
