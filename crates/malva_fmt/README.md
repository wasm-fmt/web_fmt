# malva_fmt

CSS/SCSS/SASS/LESS formatter powered by WASM ported from [malva](https://github.com/g-plane/malva).

## Usage

### Node.js

```js
import init from "malva_fmt";

const { format } = await init();

const result = format(
	".foo {\n  color: red;\n}",
	"style.css",
	{
		indentStyle: "space",
		indentWidth: 2,
		lineWidth: 80,
		lineEnding: "lf",
	}
);
```

### Vite

Add `"@wasm-fmt/malva-fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
    "optimizeDeps": {
        "exclude": ["@wasm-fmt/malva-fmt"]
    }
}
```

<details>
<summary>
If you cannot change the vite config, you can use another import entry

</summary>

```js
import init from "@wasm-fmt/malva-fmt/vite";

const { format } = await init();

const result = format(
	".foo {\n  color: red;\n}",
	"style.css",
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

See [malva configuration docs](https://github.com/g-plane/malva/blob/main/docs/config.md) for all available options.
