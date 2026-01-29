[![Test](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml)

# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/graphql_fmt)](https://www.npmjs.com/package/@wasm-fmt/graphql_fmt)

```bash
npm install @wasm-fmt/graphql_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/graphql-fmt)](https://jsr.io/@fmt/graphql-fmt)

```bash
npx jsr add @fmt/graphql-fmt
```

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/graphql_fmt";

const input = `query { user { name } }`;

const formatted = format(input);
console.log(formatted);
```

## Web

For web environments, you need to initialize WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/graphql_fmt/web";

await init();

const input = `query { user { name } }`;

const formatted = format(input);
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/graphql_fmt/vite";

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

See [pretty_graphql configuration docs](https://pretty-graphql.netlify.app/) for all available options.

# Credits

Thanks to:

- The [pretty_graphql](https://github.com/g-plane/pretty_graphql) project
