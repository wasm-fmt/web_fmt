[![Test](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml)

# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/markup_fmt)](https://www.npmjs.com/package/@wasm-fmt/markup_fmt)

```bash
npm install @wasm-fmt/markup_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/markup-fmt)](https://jsr.io/@fmt/markup-fmt)

```bash
npx jsr add @fmt/markup-fmt
```

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/markup_fmt";

const input = `<div  class="container"  id="main" >Hello World</div>`;

const formatted = format(input, "index.html");
console.log(formatted);
```

## Web

For web environments, you need to initialize WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/markup_fmt/web";

await init();

const input = `<div  class="container"  id="main" >Hello World</div>`;

const formatted = format(input, "index.html");
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/markup_fmt/vite";

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

See [markup_fmt configuration docs](https://github.com/g-plane/markup_fmt) for all available options.

# Credits

Thanks to:

- The [markup_fmt](https://github.com/g-plane/markup_fmt) project
