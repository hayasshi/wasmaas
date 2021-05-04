use std::time::SystemTime;

use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use wasmtime::*;
use wasmtime_wasi::*;

#[get("/status")]
async fn status(req: HttpRequest) -> impl Responder {
    println!("[INFO] /status: Request from {:?}", req.peer_addr());
    "OK"
}

#[get("/double/{num}")]
async fn double(req: HttpRequest) -> impl Responder {
    println!("[INFO] /double/_num: Request from {:?}", req.peer_addr());
    let num: i32 = req.match_info().get("num").unwrap().parse().unwrap();

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
async fn greet(req: HttpRequest) -> impl Responder {
    println!("[INFO] /greet/_name: Request from {:?}", req.peer_addr());
    let name: String = req.match_info().get("name").unwrap().to_string();

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

#[get("/wasi/{name}")]
async fn wasi(req: HttpRequest) -> impl Responder {
    println!("[INFO] /wasi/_name: Request from {:?}", req.peer_addr());

    let store = Store::default();
    let mut linker = Linker::new(&store);

    // let wasiCtx = WasiCtx::builder(
    //     RefCell::new(Box::new(RngCore::new())),
    //     WasiClocks::new(),
    //     Box::new(WasiSched::new()),
    //     Rc::new(RefCell::new(Table::new(&store, TableType::new(ValType::I32, 1), 1)))
    // );
    let wasi = Wasi::new(&store, WasiCtxBuilder::new().inherit_stdio().build().unwrap());
    wasi.add_to_linker(&mut linker).unwrap();

    let module = Module::from_file(store.engine(), "foo.wasm").unwrap();
    let instance = linker.instantiate(&module).unwrap();
    match instance.get_func("").unwrap().call(&[]) {
        Ok(_) => String::from("OK"),
        Err(e) => e.to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(status).service(double).service(greet))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
