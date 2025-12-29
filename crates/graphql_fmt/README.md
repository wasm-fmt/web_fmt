# graphql_fmt

GraphQL formatter powered by WASM ported from [pretty_graphql](https://github.com/g-plane/pretty_graphql).

## Usage

### Node.js

```js
import init from "graphql_fmt";

const { format } = await init();

const result = format(
	"query { user { name } }",
	"query.graphql",
	{
		indentStyle: "space",
		indentWidth: 2,
		lineWidth: 80,
		lineEnding: "lf",
	}
);
```

### Vite

Add `"@wasm-fmt/graphql-fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/graphql-fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```js
import init from "@wasm-fmt/graphql-fmt/vite";

const { format } = await init();

const result = format(
	"query { user { name } }",
	"query.graphql",
	{
		indentStyle: "space",
		indentWidth: 2,
		lineWidth: 80,
		lineEnding: "lf",
	}
);
```

</details>

## Configuration

See [pretty_graphql configuration docs](https://pretty-graphql.netlify.app/) for all available options.
