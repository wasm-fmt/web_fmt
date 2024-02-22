cd $(dirname $0)/..
crates_dir=$(pwd)

cd ../..
wasm-pack build --target=web --scope=wasm-fmt crates/json_fmt

cd $crates_dir

cp -R ./extra/. ./pkg/

./scripts/package.mjs ./pkg/package.json
