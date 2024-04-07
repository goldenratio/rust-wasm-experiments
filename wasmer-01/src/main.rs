use wasmer::{Store, Module, Instance, imports};

fn main() -> anyhow::Result<()> {
    let mut store = Store::default();
    let module = Module::from_file(&store, "./wasm_add_bg.wasm").unwrap();
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_fn = instance.exports.get_typed_function::<(u32, u32), u32>(&mut store,"add_num")?;
    let result = add_fn.call(&mut store, 1, 2)?;
    println!("{:?}", result);

    Ok(())
}