[![Test](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml)

# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/oxc_fmt)](https://www.npmjs.com/package/@wasm-fmt/oxc_fmt)

```bash
npm install @wasm-fmt/oxc_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/oxc-fmt)](https://jsr.io/@fmt/oxc-fmt)

```bash
npx jsr add @fmt/oxc-fmt
```

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/oxc_fmt";

const input = `const foo  =  "bar"`;

const formatted = format(input, "index.js");
console.log(formatted);
```

## Web

For web environments, you need to initialize WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/oxc_fmt/web";

await init();

const input = `const foo  =  "bar"`;

const formatted = format(input, "index.js");
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/oxc_fmt/vite";

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

See [oxc formatter configuration docs](https://oxc.rs/docs/guide/usage/formatter/config.html) and [prettier options](https://prettier.io/docs/options) for all available options.

# Credits

Thanks to:

- The [oxc](https://github.com/oxc-project/oxc) project
