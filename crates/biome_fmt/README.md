[![Test](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml)

# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/biome_fmt)](https://www.npmjs.com/package/@wasm-fmt/biome_fmt)

```bash
npm install @wasm-fmt/biome_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/biome-fmt)](https://jsr.io/@fmt/biome-fmt)

```bash
npx jsr add @fmt/biome-fmt
```

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/biome_fmt";

const input = `function foo() {console.log("Hello, world!")}`;

const formatted = format(input, "app.js");
console.log(formatted);
```

## Web

For web environments, you need to initialize WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/biome_fmt/web";

await init();

const input = `function foo() {console.log("Hello, world!")}`;

const formatted = format(input, "app.js");
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/biome_fmt/vite";

await init();
// ...
```

## Entry Points

- `.` - Auto-detects environment (Node.js uses node, Webpack uses bundler, default is ESM)
- `./node` - Node.js environment (no init required)
- `./esm` - ESM environments like Deno (no init required)
- `./bundler` - Bundlers like Webpack (no init required)
- `./web` - Web browsers (requires manual init)
- `./vite` - Vite bundler (requires manual init)

# Configuration

See [Biome formatter configuration docs](https://biomejs.dev/reference/configuration/#formatter) for all available options.

# Credits

Thanks to:

- The [Biome](https://github.com/biomejs/biome) project
