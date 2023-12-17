const {
  walkdir: innerWalkdir,
  walkdirWithConfig: innerWalkdirWithConfig,
} = require('./binding');

function walkdir(entry, config, callback) {
  if (!entry) return;

  const noop = () => {};

  if (arguments.length === 2) {
    innerWalkdir(entry, config || noop);
    return;
  }

  innerWalkdirWithConfig(entry, toBuffer(config), callback);
}

function toBuffer(t) {
  return Buffer.from(JSON.stringify(t));
}

module.exports.walkdir = walkdir;
