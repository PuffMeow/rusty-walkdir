const { Bench } = require('tinybench');
const path = require('path');
const walkdir = require('walkdir');
const { walkdir: rustWalkdir } = require('../index');

async function main() {
  const bench = new Bench();

  const testPath = path.join(__dirname, '../npm');
  bench
    .add('Node.js: walkdir', () => {
      walkdir(testPath, (p) => {});
    })
    .add('Rust: walkdir', () => {
      rustWalkdir(testPath, { minDepth: 1 }, (p) => {});
    });

  await bench.run();
  console.table(bench.table());
}

main();
