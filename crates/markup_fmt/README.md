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

```javascript
import init, { format } from "@wasm-fmt/markup_fmt";

await init();

const input = `<div  class="container"  id="main" >Hello World</div>`;

const formatted = format(input, "index.html");
console.log(formatted);
```

For Vite users:

Add `"@wasm-fmt/markup_fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/markup_fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```JavaScript
import init, { format } from "@wasm-fmt/markup_fmt/vite";

// ...
```

</details>
