cd program
cargo build-bpf --tools-version v1.43
shank idl
cd ../
python3 postprocess_idl.py --fp program/idl/stats_bucket.json
cp program/idl/stats_bucket.json sdk/idl
cd js
node ./kinobi.mjs