cd $(dirname $0)/..
crates_dir=$(pwd)

cd ../..
wasm-pack build --target=web --scope=wasm-fmt crates/rome_fmt
cp README.md LICENSE crates/rome_fmt/pkg/

cd $crates_dir
# backup rome_fmt.js
cp ./pkg/rome_fmt.js ./pkg/rome_fmt.js.bak

git apply ./patch/rome_fmt.patch

cp -R ./extra/. ./pkg/

./scripts/package.mjs ./pkg/package.json
