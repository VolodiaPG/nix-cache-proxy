# Nix Cache Proxy

Request Nix binary caches in parallel. Speeds up Nix daemon's binary cache lookup requests.

## Building

```bash
cargo build --release
```

The compiled binary will be available at `target/release/nix-cache-proxy`.

## Configuration

Configuration is done via environment variables:

- **`UPSTREAMS`** (required): Comma-separated list of upstream cache URLs

  - Default: `https://cache.nixos.org`
  - Example: `https://cache.nixos.org,https://attic.xuyh0120.win/lantian`

- **`BIND_ADDRESS`** (optional): Address and port to bind the server to
  - Default: `127.0.0.1:8080`
