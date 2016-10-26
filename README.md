# RCP [![Build Status](https://travis-ci.org/mrLSD/rcp.svg?branch=master)](https://travis-ci.org/mrLSD/rcp) [![License](http://img.shields.io/badge/license-mit-blue.svg?style=flat-square)](https://raw.githubusercontent.com/mrLSD/rcp/master/LICENSE)

Fast copy files based on **Rust lang**.

Main features:
* extremely fast and safely copy files
* **CLI** tool with help message and optional params
* Buffer **cli** parameter for various copying scenario.

### How to build
* If **Rust** not installed. Install Rust: `make install`
* Build release version: `make`
* Build debug version: `make build`

### How to run
* Build first
* For help: `target/release/rcp --help`
* Basic usage: `target/release/rcp inpit_file output_file`
* Custom buffer size: `target/release/rcp -b 1024 inpit_file output_file`

#### MIT LICENSE [![License](http://img.shields.io/badge/license-mit-blue.svg?style=flat-square)](https://raw.githubusercontent.com/mrLSD/rcp/master/LICENSE)
