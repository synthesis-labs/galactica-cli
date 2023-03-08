# cargo clean
PACKAGE_VERSION=$(cargo metadata --format-version=1 | jq --raw-output '.packages | [.[] | select(.name=="galactica") ] | .[0].version')
GIT_VERSION=build.$(git rev-list --all --count).$(git rev-parse --short HEAD)
VER=$PACKAGE_VERSION-$GIT_VERSION
echo Version is $VER

if [[ $(git diff --stat) != '' ]]; then
  echo 'Working directory is dirty - wont build!'
  exit 1
else
  echo 'Clean LGTM!'
fi

cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu
# cargo build --release --target x86_64-unknown-linux-gnu

tar cvfz bin/galactica-x86_64-apple-darwin.tar.gz -C target/x86_64-apple-darwin/release galactica
tar cvfz bin/galactica-aarch64-apple-darwin.tar.gz -C target/aarch64-apple-darwin/release galactica
zip -j bin/galactica-x86_64-pc-windows-gnu.zip target/x86_64-pc-windows-gnu/release/galactica.exe

shasum -a 256 bin/galactica-x86_64-apple-darwin.tar.gz
shasum -a 256 bin/galactica-aarch64-apple-darwin.tar.gz
shasum -a 256 bin/galactica-x86_64-pc-windows-gnu.zip