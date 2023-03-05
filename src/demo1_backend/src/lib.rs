#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    
    ic_cdk::api::print(format!("Hello from IC debugger: {}", name));
    println!("Hello from WASI: {}", name);

    format!("Hello, {}!", name)
}

#[ic_cdk_macros::init]
fn init() {
    ic_polyfill::init();
}

