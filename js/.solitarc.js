const path = require('path');
const programDir = path.join(__dirname, '..', 'program');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'shank',
  programName: 'stats_bucket',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
  typeAliases: {
    number: 'f32'
  }
};