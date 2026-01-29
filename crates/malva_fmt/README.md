[![Test](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml)

# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/malva_fmt)](https://www.npmjs.com/package/@wasm-fmt/malva_fmt)

```bash
npm install @wasm-fmt/malva_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/malva-fmt)](https://jsr.io/@fmt/malva-fmt)

```bash
npx jsr add @fmt/malva-fmt
```

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/malva_fmt";

const input = `.foo { color: red; }`;

const formatted = format(input, "style.css");
console.log(formatted);
```

## Web

For web environments, you need to initialize WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/malva_fmt/web";

await init();

const input = `.foo { color: red; }`;

const formatted = format(input, "style.css");
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/malva_fmt/vite";

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

See [malva configuration docs](https://github.com/g-plane/malva/blob/main/docs/config.md) for all available options.

# Credits

Thanks to:

- The [malva](https://github.com/g-plane/malva) project
