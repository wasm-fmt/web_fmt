[![npm](https://img.shields.io/npm/v/@wasm-fmt/json_fmt)](https://www.npmjs.com/package/@wasm-fmt/json_fmt)

# Install

```bash
npm install @wasm-fmt/json_fmt
```

# Usage

```javascript
import init, { format } from "@wasm-fmt/json_fmt";

await init();

const input = `{"hello":"world"}`;

const formatted = format(input);
console.log(formatted);
```

For Vite users:

```JavaScript
import init, { format } from "@wasm-fmt/json_fmt/vite";

// ...
```
