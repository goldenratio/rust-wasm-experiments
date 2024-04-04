use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

fn main() {
    let engine = Engine::default();

    let module = Module::from_file(&engine, "./wasm_add_bg.wasm").unwrap();

    let mut exports = module.exports();
    while let Some(foo) = exports.next() {
        println!("{}", foo.name());
    }

    let linker = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
    let mut store = Store::new(&engine, wasi);

    let link = linker.instantiate(&mut store, &module).unwrap();
    let add_fn = link
        .get_typed_func::<(u32, u32), u32>(&mut store, "add_num")
        .unwrap();

    println!("{}", add_fn.call(&mut store, (1, 2)).unwrap());
}
