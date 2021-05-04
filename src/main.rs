use std::time::SystemTime;

use actix_web::{
    get,
    web::{self},
    App, HttpServer, Responder,
};
use wasmtime::*;

#[get("/double/{num}")]
async fn double(web::Path(num): web::Path<i32>) -> impl Responder {
    println!("--- start with num={}", num);
    let start = SystemTime::now();

    let store = Store::default();
    println!("Get store: {} ms", start.elapsed().unwrap().as_millis());

    let module = Module::from_file(store.engine(), "double.wasm").unwrap();
    println!("Get module: {} ms", start.elapsed().unwrap().as_millis());

    let instance = Instance::new(&store, &module, &[]).unwrap();
    println!("Get instance: {} ms", start.elapsed().unwrap().as_millis());

    let func = instance.get_typed_func::<i32, i32>("double").unwrap();
    println!("Get func: {} ms", start.elapsed().unwrap().as_millis());

    let result = func.call(num).unwrap();
    println!("Call WASM: {} ms", start.elapsed().unwrap().as_millis());

    println!("--- end\n");
    format!("Answer is {}", result)
}

/// Interface Types をつかった WASM の実行サンプル
/// `wasmtime`では機能が一時的にオミットされているため、Module を Compile 時に下記のような Err となる。(`--release`ビルドしていない WASM の場合)
/// ```
/// thread 'actix-rt:worker:0' panicked at 'called `Result::unwrap()` on an `Err` value: WebAssembly failed to compile
///
/// Caused by:
/// 0: WebAssembly translation error
/// 1: Unsupported feature: Support for interface types has temporarily been removed from `wasmtime`.
///
///    For more information about this temoprary you can read on the issue online:
///
///        https://github.com/bytecodealliance/wasmtime/issues/1271
///
///    and for re-adding support for interface types you can see this issue:
///
///        https://github.com/bytecodealliance/wasmtime/issues/677
///    ', src/main.rs:12:68
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
/// ```
#[get("/greet/{name}")]
async fn greet(web::Path(name): web::Path<String>) -> impl Responder {
    let store = Store::default();

    let module = Module::from_file(store.engine(), "greet.wasm").unwrap(); // occurred error here
    let instance = Instance::new(&store, &module, &[]).unwrap();
    let greet = instance.get_func("greet").unwrap();
    let typed_greet = greet
        .typed::<Option<ExternRef>, Option<ExternRef>>()
        .unwrap();
    let result = typed_greet.call(Some(ExternRef::new(name))).unwrap();
    match result {
        None => String::from("Missing WASM called."),
        Some(eref) => format!("{:?}", eref.data()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(double).service(greet))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
