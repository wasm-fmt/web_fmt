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

```javascript
import init, { format } from "@wasm-fmt/biome_fmt";

await init();

const input = `function foo() {console.log("Hello, world!")}`;

const formatted = format(input, "app.js");
console.log(formatted);
```

For Vite users:

Add `"@wasm-fmt/biome_fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/biome_fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```JavaScript
import init, { format } from "@wasm-fmt/biome_fmt/vite";

// ...
```

</details>
