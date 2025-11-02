# Temperature and Length Unit Conversion Application

### Run program using binary

```rust
cargo build --release
cp .\target\release\unitconv.exe .
./unitconv.exe
```

### Run program rust

### Information about subcommand

```rust
cargo run convert --from "cm" --to "inch" --value 100
cargo run convert --from "cm" --to "km" --value 100
cargo run convert --from "inch" --to "km" --value 100
cargo run list
cargo run history
cargo run -- help done
cargo run -- help remove
```

### Release app

```rust
cargo build --release
```

### Make sure the app run is well

```rust
# Linux/macOS
./target/release/unitconv --help


# Windows (PowerShell)
.\target\release\unitconv.exe --help
```
