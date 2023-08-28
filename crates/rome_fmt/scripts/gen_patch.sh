# assert current dir is crates/rome_fmt
cd $(dirname $0)/..
crates_dir=$(pwd)

cd ./pkg
pkg_prefix=$(git rev-parse --show-prefix)

tmp_dir=$(mktemp -d)

cd $tmp_dir
git init

cp $crates_dir/pkg/rome_fmt.js.bak $tmp_dir/rome_fmt.js
git add -f .
git commit -m "init"

cp $crates_dir/pkg/rome_fmt.js $tmp_dir/rome_fmt.js
git add -f .

git diff \
    --cached \
    --no-color \
    --ignore-space-at-eol \
    --no-ext-diff \
    --src-prefix=a/$pkg_prefix \
    --dst-prefix=b/$pkg_prefix \
    >$crates_dir/patch/rome_fmt.patch

rm -rf $tmp_dir
