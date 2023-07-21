# demo1 - Basic "Hello World" demonstration project

This is a "hello world" demo project that can be compiled to `wasm32-wasi` target and run in dfx.

It is assumed that you have [rust](https://doc.rust-lang.org/book/ch01-01-installation.html), [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/), and [wasi2ic](https://github.com/wasm-forge/wasi2ic) installed.


You can now either create this project from scratch or clone this repository and skip to the "Deployment and testing" stage.


## Creating the project from scratch

* You need to create the project using `dfx new --type rust --no-frontend demo1`
* Go to folder: `demo1/src/demo1_backend`
* Add the ic_polyfill dependency: `cargo add --git https://github.com/wasm-forge/ic-wasi-polyfill`

* Modify the `demo1/src/demo1_backend/src/lib.rs` file containing the `greet` method so that it outputs messages to the debug console and uses the 'println!' command:

```rust
#[ic_cdk::query]
fn greet(name: String) -> String {
    
    ic_cdk::api::print(format!("Hello from IC debugger: {}", name));
    println!("Hello from WASI: {}", name);

    format!("Hello, {}!", name)
}

#[ic_cdk::init]
fn init() {
    unsafe {ic_wasi_polyfill::init(&[0u8;32], &[]);}
}

```


## Deployment and testing

Start the `dfx` environment in a separate console:
```
  dfx start
```
This window will show you the communication with your canister.


Once you have the demo1 project in your folder, enter the folder and deploy the project using:

```bash
  dfx canister create --all
```

You have to build the project inside the `demo1` folder for a `wasm32-wasi` target (you might need to install the target using rustup), do this with the command:
```bash
  cargo build --release --target wasm32-wasi
```

If everything works out, you can now enter the new folder `target/wasm32-wasi/release`, it should contain the file: 'demo1_backend.wasm'.

This file cannot be deployed directly because it has WASI dependencies in it. 
(You can check that by converting the .wasm file to its textual representation with the `wasm2wat` command).

Now, use the `wasi2ic` tool to re-route the dependencies:
```bash
  wasi2ic demo1_backend.wasm no_wasi.wasm
```
It creates a new file called `no_wasi.wasm`. Now you can deploy it manually:

```bash
  dfx canister install --mode reinstall --wasm no_wasi.wasm demo1_backend
```

Accept the canister overwrite with 'yes'. Now call the canister:

```bash
  dfx canister call demo1_backend greet test_hello
```

You should now in the console window see the greeting from the debug output as well as the greeting from WASI.

