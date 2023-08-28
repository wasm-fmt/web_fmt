[![Test](https://github.com/wasm-fmt/rome_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/rome_fmt/actions/workflows/test.yml)
[![npm](https://img.shields.io/npm/v/@wasm-fmt/rome_fmt)](https://www.npmjs.com/package/@wasm-fmt/rome_fmt)

# Install

```bash
npm install @wasm-fmt/rome_fmt
```

# Usage

```javascript
import init, { format } from "@wasm-fmt/rome_fmt";

await init();

const input = `function foo() {console.log("Hello, world!")}`;

const formatted = format(input);
console.log(formatted);
```

For Vite users:

```JavaScript
import init, { format } from "@wasm-fmt/rome_fmt/vite";

// ...
```
