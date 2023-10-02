# assert current dir is crates/biome_fmt
cd $(dirname $0)/..
crates_dir=$(pwd)

cd ./pkg
pkg_prefix=$(git rev-parse --show-prefix)

tmp_dir=$(mktemp -d)

cd $tmp_dir
git init

cp $crates_dir/pkg/biome_fmt.js.bak $tmp_dir/biome_fmt.js
git add -f .
git commit -m "init"

cp $crates_dir/pkg/biome_fmt.js $tmp_dir/biome_fmt.js
git add -f .

git diff \
    --cached \
    --no-color \
    --ignore-space-at-eol \
    --no-ext-diff \
    --src-prefix=a/$pkg_prefix \
    --dst-prefix=b/$pkg_prefix \
    >$crates_dir/patch/biome_fmt.patch

rm -rf $tmp_dir
