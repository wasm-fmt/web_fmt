cd $(dirname $0)/..
crates_dir=$(pwd)

cd ../..
wasm-pack build --target=web --scope=wasm-fmt crates/biome_fmt

cd $crates_dir
# backup biome_fmt.js
cp ./pkg/biome_fmt.js ./pkg/biome_fmt.js.bak

git apply ./patch/biome_fmt.patch

cp -R ./extra/. ./pkg/

./scripts/package.mjs ./pkg/package.json
