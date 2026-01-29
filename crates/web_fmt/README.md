[![Test](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/web_fmt/actions/workflows/test.yml)

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
- **GraphQL**

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/web_fmt";

// JavaScript/TypeScript
format(`function foo() {console.log("Hi")}`, "index.js");

// CSS/SCSS/LESS
format(`.foo{color:red}`, "style.css");

// HTML/Vue/Svelte
format(`<div  class="x">Hi</div>`, "index.html");

// JSON
format(`{"name":"John"}`, "data.json");

// GraphQL
format(`query{user{name}}`, "query.graphql");
```

## Web

For web environments, you need to initialize WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/web_fmt/web";

await init();

// Format your code
const formatted = format(`function foo() {console.log("Hi")}`, "index.js");
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/web_fmt/vite";

await init();
// ...
```

## Entry Points

- `.` - Auto-detects environment (Node.js uses node, Webpack uses bundler, default is ESM)
- `./node` - Node.js environment (no init required)
- `./esm` - ESM environments like Deno (no init required)
- `./bundler` - Bundlers like Webpack (no init required)
- `./web` - Web browsers (requires manual init)
- `./vite` - Vite bundler (requires manual init)

## With Configuration

```javascript
const config = {
	// Common layout options
	indentStyle: "space", // "tab" | "space"
	indentWidth: 4, // number
	lineWidth: 100, // number
	lineEnding: "lf", // "lf" | "crlf"

	// Format-specific options
	script: { quoteStyle: "single", semiColons: "asNeeded" },
	style: { declarationOrder: "alphabetical" },
	markup: { selfClosingSpace: false },
	json: { trailingComma: true },
};

format(code, filename, config);
```

# Configuration

- [Biome](https://biomejs.dev/reference/configuration/#formatter) (Script)
- [Malva](https://github.com/g-plane/malva/blob/main/docs/config.md) (Style)
- [markup_fmt](https://github.com/g-plane/markup_fmt) (Markup)
- [pretty-graphql](https://pretty-graphql.netlify.app/) (GraphQL)

# Credits

Thanks to:

- The [Biome](https://github.com/biomejs/biome) project
- The [malva](https://github.com/g-plane/malva) project
- The [markup_fmt](https://github.com/g-plane/markup_fmt) project
- The [pretty_graphql](https://github.com/g-plane/pretty_graphql) project
