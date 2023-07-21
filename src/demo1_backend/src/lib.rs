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


