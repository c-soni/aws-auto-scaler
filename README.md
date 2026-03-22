```bash
cargo build --release
REDIS_URL="redis://127.0.0.1" cargo run --release 2> output.log
```

For local testing, you can use `podman run -p 6379:6379 -it redis`