const { walkdir } = require("./index");
const path = require("path");

let entry = path.join(process.cwd(), "node_modules");
walkdir(entry, (p) => {
  console.log("path:", p);
});
