# @daydog/toml

A TOML parsing tool written in Rust for Node.js

[![Build Status](https://github.com/day-dog/node-toml/actions/workflows/CI.yml/badge.svg)](https://github.com/day-dog/node-toml/actions/workflows/CI.yml/badge.svg)

[![NPM](https://nodei.co/npm/@daydog/toml.png?downloads=true)](https://nodei.co/npm/@daydog/toml)

## Installation

@daydog/toml is available via npm.

    npm install @daydog/toml

## Usage

### parse

You can convert toml strings to JSON both synchronously and asynchronously.

```javascript
const { parseSync } = require('@daydog/toml')
var data = parseSync(`
                    [test]
                    foo = "bar"
                    `)
console.log(data)
```

```javascript
const { pasre } = require('@daydog/toml')
pasre(`
       [test]
       foo = "bar"
       `).then(res => console.log(res))

```

## License

toml-node is licensed under the GPL-3.0 license agreement. See the LICENSE file for more information.
