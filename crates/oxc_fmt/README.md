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

```javascript
import init, { format } from "@wasm-fmt/oxc_fmt";

await init();

const input = `const foo  =  "bar"`;

const formatted = format(input, "index.js");
console.log(formatted);
```

For Vite users:

Add `"@wasm-fmt/oxc_fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/oxc_fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```JavaScript
import init, { format } from "@wasm-fmt/oxc_fmt/vite";

// ...
```

</details>

# Configuration

See [oxc formatter configuration docs](https://oxc.rs/docs/guide/usage/formatter/config.html) and [prettier options](https://prettier.io/docs/options) for all available options.
