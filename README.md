# demo1 - Basic "Hello World" demonstration project

This is a "hello world" demo project that can be compiled to `wasm32-wasi` target and run in dfx.

It is assumed that you have [rust](https://doc.rust-lang.org/book/ch01-01-installation.html), [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/), and [wasi2ic](https://github.com/wasm-forge/wasi2ic) installed.


You can now either create this project from scratch or clone this repository and skip to the "Deployment and testing" stage.


## Creating the project from scratch

* You need to create the project using `dfx new --type rust --no-frontend demo1`
* Go to the backend source folder folder: `cd src/demo1_backend`
* Add the stable memory dependency: `cargo add ic-stable-structures` -- this is to show how to work with the custom memory
* Add the polyfill dependency: `cargo add ic-wasi-polyfill`

* Modify the `demo1/src/demo1_backend/src/lib.rs` file containing the `greet` method so that it outputs messages to the debug console and uses the 'println!' command:

```rust
use std::cell::RefCell;
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager}, DefaultMemoryImpl};

// WASI polyfill requires a virtual stable memory to store the file system.
// You can replace `0` with any index up to `254`.
const WASI_MEMORY_ID: MemoryId = MemoryId::new(0);

thread_local! {
    // The memory manager is used for simulating multiple memories.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    
    ic_cdk::api::print(format!("Hello from IC debugger: {}", name));
    println!("Hello from WASI: {}", name);

    format!("Hello, {}!", name)
}

#[ic_cdk::init]
fn init() {
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(WASI_MEMORY_ID));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(WASI_MEMORY_ID));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);    
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

