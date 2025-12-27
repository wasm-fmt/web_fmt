# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/web_fmt)](https://www.npmjs.com/package/@wasm-fmt/web_fmt)

```bash
npm install @wasm-fmt/web_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/web-fmt)](https://jsr.io/@fmt/web-fmt)

```bash
npx jsr add @fmt/web-fmt
```

# Supported Formats

- **Script**: JavaScript, TypeScript, JSX and TSX
- **Style**: CSS, SASS, LESS
- **Markup**: HTML, Vue, Svelte, Astro, Jinja, Twig
- **JSON**: JSON, JSON with comments

# Usage

```javascript
import init, { format } from "@wasm-fmt/web_fmt";

await init();

const input = `function foo() {console.log("Hello, world!")}`;

const formatted = format(input, "index.js");
console.log(formatted);
```

For Vite users:

Add `"@wasm-fmt/web_fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/web_fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```JavaScript
import init, { format } from "@wasm-fmt/web_fmt/vite";

// ...
```

</details>