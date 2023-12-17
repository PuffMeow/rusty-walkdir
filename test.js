const { walkdir } = require('./index');
const walkdirNode = require('walkdir');
const path = require('path');
const fs = require('fs');

let entry = path.join(process.cwd(), 'npm');
walkdirNode(entry, (visit) => {
  fs.appendFileSync(
    path.join(__dirname, './node_test.txt'),
    `${visit}\n`,
    'utf-8'
  );
});

walkdir(entry, { minDepth: 1 }, (visit) => {
  fs.appendFileSync(
    path.join(__dirname, './rust_test.txt'),
    `${visit}\n`,
    'utf-8'
  );
});
