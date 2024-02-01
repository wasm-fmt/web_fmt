[![npm](https://img.shields.io/npm/v/@wasm-fmt/web_fmt)](https://www.npmjs.com/package/@wasm-fmt/web_fmt)

# Install

```bash
npm install @wasm-fmt/web_fmt
```

# Usage

```javascript
import init, { format } from "@wasm-fmt/web_fmt";

await init();

const input = `function foo() {console.log("Hello, world!")}`;

const formatted = format(input, "index.js");
console.log(formatted);
```

For Vite users:

```JavaScript
import init, { format } from "@wasm-fmt/web_fmt/vite";

// ...
```
