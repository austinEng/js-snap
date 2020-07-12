# JS Snap
JS Snap is a utility to produce a static or shared library from a bundled JavaScript file.

JS Snap embeds V8 in order to execute the bundled JavaScript. The main use case for JS Snap over Node.js is that the generated library may be linked into another binary and used without taking a runtime dependency on Node.js. The JavaScript bundle may be embedded into the V8 snapshot to reduce startup time of parsing and interpreting the JavaScript code.

JS Snap is useful when it is desirable to re-use JavaScript code from non-JS code. For example, a JavaScript web app could be bundled, js-snap'ed, and linked into Go or Rust code to do server-side rendering. Please see https://github.com/austinEng/react-ssr-go for an example of this.
