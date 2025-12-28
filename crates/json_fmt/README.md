# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/json_fmt)](https://www.npmjs.com/package/@wasm-fmt/json_fmt)

```bash
npm install @wasm-fmt/json_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/json-fmt)](https://jsr.io/@fmt/json-fmt)

```bash
npx jsr add @fmt/json-fmt
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

Add `"@wasm-fmt/json_fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/json_fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```JavaScript
import init, { format } from "@wasm-fmt/json_fmt/vite";

// ...
```

</details>
