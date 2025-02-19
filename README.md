# Serbian Transliteration

A fast, Rust-powered WASM library for Serbian Latin to Cyrillic (and vice versa) transliteration.

## Features

- Convert Serbian Latin text to Cyrillic
- Convert Serbian Cyrillic text to Latin
- Fast performance (powered by Rust and WebAssembly)
- Works in both browser and Node.js environments
- TypeScript support

## Installation

## Setup

### For Vite Projects

If you're using Vite, you'll need to install and configure vite-plugin-wasm:

`npm install -D vite-plugin-wasm`

Then add this to your `vite.config.js` or `vite.config.ts`:

```javascript
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
export default defineConfig({
  plugins: [wasm()],
});
```

### For Webpack Projects

If you're using webpack, add this to your `webpack.config.js`:

```javascript
module.exports = {
  // ... other config
  experiments: {
    asyncWebAssembly: true,
  },
};
```

## Usage

```javascript
import { Transliterator } from "serbian-transliteration";
// Create a new transliterator instance
const transliterator = new Transliterator();
// Convert Latin to Cyrillic
console.log(transliterator.to_cyrillic("njegov čaj")); // његов чај
console.log(transliterator.to_cyrillic("život i priključenja")); // живот и прикључења
console.log(transliterator.to_cyrillic("ljulja se ljuljaška")); // љуља се љуљашка
// Convert Cyrillic to Latin
console.log(transliterator.to_latin("његов чај")); // njegov čaj
console.log(transliterator.to_latin("живот и прикључења")); // život i priključenja
console.log(transliterator.to_latin("љуља се љуљашка")); // ljulja se ljuljaška
```
