# Getting pyodide working with embedded deno

## Install

1. `npm install`
2. `cargo run`

This will run the rust project that will fire up deno, but fails to start pyodide.

Code comes from:

* https://til.simonwillison.net/deno/pyodide-sandbox
* https://github.com/rscarson/rustyscript/blob/master/examples/node_import/main.rs

`runner.ts` is from the first link, and it works from the command line:

```bash
 $ deno run --allow-read runner.ts
{"code":"4+6"}
{"output":10}
{"shutdown":true}
```

Running the built app results in the following:
```
/Users/mlilback/working/notecalc/erunner/target/debug/erunner
Getting hostname from node:os:
tiger
Hello from rusty
loading test2.js
hello() called
loading pyodide
Error: TypeError: core.ops.op_main_module is not a function
    at Object.mainModule (ext:init_runtime/init_runtime.js:31:43)
    at Array.get (file:///Users/mlilback/working/notecalc/erunner/node:process:912:14)
    at file:///Users/mlilback/working/notecalc/erunner/node_modules/pyodide/pyodide.asm.js:10:1626
    at loadPyodide (file:///Users/mlilback/working/notecalc/erunner/node_modules/pyodide/pyodide.js:3:9854)
    at async Module.hello (file:///Users/mlilback/working/notecalc/erunner/test2.js:9:37)
```
