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

## Basic Usage

```javascript
import init, { format } from "@wasm-fmt/web_fmt";

await init();

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

## Vite Configuration

Add `"@wasm-fmt/web_fmt"` to `optimizeDeps.exclude`:

```JSON
{
  "optimizeDeps": {
    "exclude": ["@wasm-fmt/web_fmt"]
  }
}
```

Or use the vite entry:

```javascript
import init, { format } from "@wasm-fmt/web_fmt/vite";
```

## Format-Specific Options

- [Biome](https://biomejs.dev/reference/configuration/#formatter) (Script)
- [Malva](https://github.com/g-plane/malva/blob/main/docs/config.md) (Style)
- [markup_fmt](https://github.com/g-plane/markup_fmt) (Markup)
- [pretty-graphql](https://pretty-graphql.netlify.app/) (GraphQL)
