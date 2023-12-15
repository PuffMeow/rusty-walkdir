const { walkdir: _walkdir } = require('./binding');

function walkdir(entry, callback) {
  const noop = () => {};
  if (!entry) return;
  _walkdir(entry, callback || noop);
}

module.exports.walkdir = walkdir;
