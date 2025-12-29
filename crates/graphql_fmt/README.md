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

```javascript
import init, { format } from "@wasm-fmt/graphql_fmt";

await init();

const input = `query { user { name } }`;

const formatted = format(input);
console.log(formatted);
```

For Vite users:

Add `"@wasm-fmt/graphql_fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/graphql_fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```JavaScript
import init, { format } from "@wasm-fmt/graphql_fmt/vite";

// ...
```

</details>

# Configuration

See [pretty_graphql configuration docs](https://pretty-graphql.netlify.app/) for all available options.
