# cargo clean
PACKAGE_VERSION=$(cargo metadata --format-version=1 | jq --raw-output '.packages | [.[] | select(.name=="galactica") ] | .[0].version')
GIT_VERSION=build.$(git rev-list --all --count).$(git rev-parse --short HEAD)
VER=$PACKAGE_VERSION+$GIT_VERSION
echo Version is $VER

if [[ $(git diff --stat) != '' ]]; then
  echo 'Working directory is dirty - wont build!'
  # exit 1
else
  echo 'Clean LGTM!'
fi
    
    # aarch64-apple-darwin \
    # x86_64-apple-darwin \
    # x86_64-pc-windows-gnu \

for target in \
    x86_64-unknown-linux-musl \
    ;
do
  echo building $target ...
  docker run --rm \
      --volume "${PWD}":/root/src \
      --workdir /root/src \
        7c0307363b8a05478dab58c73bce99e397a975a86e005ee62aa52821e985accd \
          sh -c "cargo build --release --target $target"
done

# docker run --rm \
#     --volume "${PWD}":/root/src \
#     --workdir /root/src \
#       joseluisq/rust-linux-darwin-builder:1.68.0 \
#         sh -c "cargo build --release --target aarch64-apple-darwin"

# docker run --rm \
#     --volume "${PWD}":/root/src \
#     --workdir /root/src \
#       joseluisq/rust-linux-darwin-builder:1.68.0 \
#         sh -c "cargo build --release --target x86_64-apple-darwin"

# docker run --rm \
#     --volume "${PWD}":/root/src \
#     --workdir /root/src \
#       joseluisq/rust-linux-darwin-builder:1.68.0 \
#         sh -c "cargo build --release --target x86_64-pc-windows-gnu"

# docker run --rm \
#     --volume "${PWD}":/root/src \
#     --workdir /root/src \
#       joseluisq/rust-linux-darwin-builder:1.68.0 \
#         sh -c "cargo build --release --target x86_64-unknown-linux-gnu"

# cargo build --release --target x86_64-apple-darwin
# cargo build --release --target aarch64-apple-darwin
# cargo build --release --target x86_64-pc-windows-gnu
# # cargo build --release --target x86_64-unknown-linux-gnu

# tar cvfz bin/galactica-x86_64-apple-darwin-$VER.tar.gz -C target/x86_64-apple-darwin/release galactica
# tar cvfz bin/galactica-aarch64-apple-darwin-$VER.tar.gz -C target/aarch64-apple-darwin/release galactica
# zip -j bin/galactica-x86_64-pc-windows-gnu-$VER.zip target/x86_64-pc-windows-gnu/release/galactica.exe

# shasum -a 256 bin/galactica-x86_64-apple-darwin-$VER.tar.gz bin/galactica-x86_64-apple-darwin-$VER.tar.gz.sha256
# shasum -a 256 bin/galactica-aarch64-apple-darwin-$VER.tar.gz bin/galactica-aarch64-apple-darwin-$VER.tar.gz.sha256
# shasum -a 256 bin/galactica-x86_64-pc-windows-gnu-$VER.zip bin/galactica-x86_64-pc-windows-gnu-$VER.zip.sha256