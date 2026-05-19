cargo fmt
cargo clippy --all-targets --all-features
cargo check --target x86_64-unknown-linux-gnu --all-targets
cargo check --target x86_64-apple-darwin --all-targets
cargo check --target x86_64-pc-windows-msvc --all-targets
