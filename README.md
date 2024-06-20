# node-rdev

## Overview

`node-rdev` is a Node.js native addon that listens to system-wide keyboard and mouse events. It leverages the `rdev` Rust crate for event handling and provides the events to Node.js through a callback function.

## Installation

To install the module, use npm:

```sh
npm install node-rdev
```

## Usage

The primary function provided by this module is `start_listener`. This function starts listening to keyboard and mouse events and passes them to the provided callback function.

### Example

Here's a simple example of how to use `node-rdev`:

```javascript
const { start_listener } = require('node-rdev');

start_listener((event) => {
  console.log('Event received:', event);
});
```

## Tested Node Versions

The following Node.js versions have been tested with this module:

- Node.js 20
- Electron 30

## Platforms Supported

The following platforms are supported:

| Platform              | Filename                        |
| --------------------- | ------------------------------- |
| macOS (ARM64)         | node-rdev.darwin-arm64.node     |
| macOS (x64)           | node-rdev.darwin-x64.node       |
| Windows (ARM64, MSVC) | node-rdev.win32-arm64-msvc.node |
| Windows (x64, MSVC)   | node-rdev.win32-x64-msvc.node   |

## Function Documentation

### `start_listener(callback: JsFunction) -> Result<()>`

Starts listening to system-wide keyboard and mouse events.

- `callback`: A JavaScript function that will be called with the event data as its argument.

The event data is provided as a JSON string with the following structure:

```json
{
  "event_type": "EventType",
  "name": "OptionalName",
  "time": "EventTime",
  "data": "EventData"
}
```

## Event Types

The following event types are supported:

- `KeyPress`: A key was pressed.
- `KeyRelease`: A key was released.
- `MouseMove`: The mouse was moved.
- `ButtonPress`: A mouse button was pressed.
- `ButtonRelease`: A mouse button was released.
- `Wheel`: The mouse wheel was scrolled.

## Example

```javascript
const { start_listener } = require('node-rdev');

start_listener((event) => {
  console.log('Event received:', event);
});
```

## License

This project is licensed under the MIT License.
