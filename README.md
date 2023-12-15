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

walkdir('../', (path) => {
  console.log(path);
});
```
