## Introduction

Walkdir for Node.js developed by Rust.

## Installation

### npm

```
npm install @puffmeow/rusty-walkdir
```

### pnpm

```
pnpm install @puffmeow/rusty-walkdir
```

## Usage

```js
const { walkdir } = require('@puffmeow/rusty-walkdir');

// Traverse your node_modules directory
walkdir('node_modules', (path) => {
  console.log(path);
});

walkdir('../', { followSymlinks: true }, (path) => {
  console.log(path);
});
```

## Config

| property       | default  | required | description               |
| -------------- | -------- | -------- | ------------------------- |
| followSymlinks | false    | ×        | follow symlinks           |
| maxDepth       | 2^32 - 1 | x        | maximum depth to traverse |
| minDepth       | 0        | x        | minimum depth             |
