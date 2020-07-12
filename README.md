# smartcrop-wasm

[![npm version](https://img.shields.io/npm/v/smartcrop-wasm.svg)](https://www.npmjs.com/package/smartcrop-wasm)
[![install size](https://packagephobia.now.sh/badge?p=smartcrop-wasm)](https://packagephobia.now.sh/result?p=smartcrop-wasm)

WebAssembly implementation of [smartcrop.js](https://github.com/jwagner/smartcrop.js/).


## Install

```bash
# npm
$ npm install smartcrop-wasm

# or yarn
$ yarn add smartcrop-wasm
```


## Usage

```ts
import * as smartcrop from 'smartcrop-wasm';

const image = await fetch(img.src)
  .then((res) => res.arrayBuffer())
  .then((res) => new Uint8Array(res));

const result = smartcrop.crop(image, 100, 100);
// => [ 0, 25, 200, 200 ]
```


## API

### crop(image, width, height)

Find the best crop for image.

#### args

##### image: Uint8Array

The image data converted Uint8Array.

##### width: number

Crop width.

##### height: number

Crop height.

#### return: [number, number, number, number]

Return [x, y, width, height] result.


## Development

```bash
# build for wasm
$ wasm-pack build

# npm package files
$ tree pkg
pkg/
├── package.json
├── README.md
├── smartcrop_wasm_bg.d.ts
├── smartcrop_wasm_bg.js
├── smartcrop_wasm_bg.wasm
├── smartcrop_wasm.d.ts
└── smartcrop_wasm.js
```
