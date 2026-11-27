# Source 2 Demo Inspector

Browser-based replay inspection for Dota 2, Deadlock, and CS2. Browse demo messages, entities, string tables, and game events. Replays are processed locally in the browser.

## Development

Requires Node.js 22.12 or newer, Rust, and `wasm-pack`.

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.1
npm ci
npm run dev
```

## Checks

```bash
npm run lint
npm run build
```

The production build is written to `dist/`.

## License

Licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
