use std::time::Duration;
use rustyscript::{tokio, Error, Module, Runtime, RuntimeOptions};
use rustyscript::deno_core::error::AnyError;
use rustyscript::deno_core::PollEventLoopOptions;

// #[tokio::main(flavor = "multi_thread", worker_threads = 16)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    // rustyscript::init_platform(16, true);
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
    Ok(())
}

async fn run() -> Result<(), Error> {
    let module = Module::new(
        "test.js",
        r#"
            // From the node standard library (Deno polyfills)
            import os from "os";

            // From npm
            import chalk from "npm:chalk@5";

            export function print_hostname() {
                console.log("Getting hostname from node:os:");
                console.log(chalk.blue(os.hostname()));
            }
        "#,
    );

    let module2 = Module::new(
        "test2.js",
        r#"
            console.log('loading test2.js');
            import pyodideModule from "npm:pyodide/pyodide.js";
            export async function hello() {
                console.log("hello() called");
                try {
                    const { loadPyodide } = pyodideModule;
                    console.log("loading pyodide");
                    const pyodide = await loadPyodide();
                    console.log("pyodide loaded");
                    const result = await pyodide.runPythonAsync('print("Hello from python")');
                    console.log("result:", result);
                    return result;
                } catch (error) {
                    console.error("Error:", error);
                    return "ERROR";
                }
            }
        "#,
    );
    let mut runtime = Runtime::new(RuntimeOptions::default())?;
    let tokio_runtime = runtime.tokio_runtime();
    let module_handle = runtime.load_module_async(&module).await?;
    runtime.await_event_loop(Default::default(), Some(Duration::from_secs(4))).await?;
    runtime.call_function_immediate::<()>(Some(&module_handle), "print_hostname", &())?;
    runtime.eval_immediate::<()>("console.log('Hello from rusty')").await?;
    let module_handle2 = runtime.load_module_async(&module2).await?;
    runtime.await_event_loop(Default::default(), Some(Duration::from_secs(4))).await?;

    // The following two fail with "Cannot start a runtime from within a runtime"
    // runtime.call_function::<()>(Some(&module_handle2), "hello", &())?;

    // The following fails with -- assertion failed: Handle::current().runtime_flavor() == RuntimeFlavor::CurrentThread
    //
    runtime.call_function_async::<()>(Some(&module_handle2), "hello", &()).await?;
    // runtime.call_function_immediate::<()>(Some(&module_handle2), "hello", &())?;

    // runtime.await_event_loop(Default::default(), Some(Duration::from_secs(4))).await?;
    Ok(())
}
