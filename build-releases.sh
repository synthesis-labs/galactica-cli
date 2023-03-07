cargo clean
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
# cargo build --release --target x86_64-unknown-linux-gnu
# cargo build --release --target x86_64-pc-windows-gnu

tar cvfz bin/galactica-x86_64-apple-darwin.tar.gz -C target/x86_64-apple-darwin/release galactica
tar cvfz bin/galactica-aarch64-apple-darwin.tar.gz -C target/aarch64-apple-darwin/release galactica