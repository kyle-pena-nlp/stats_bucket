cd program
cargo build-bpf --tools-version v1.43
shank idl
cd ../
cp program/idl/stats_bucket.json js/idl
cd js
yarn solita
