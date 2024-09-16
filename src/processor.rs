use wasmtime::{self, Store};

pub fn process() -> anyhow::Result<()> {
    let engine = wasmtime::Engine::default();
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))
            (func (export "hello")
                (call $host_hello (i32.const 3))
            )
        )
    "#;

    let module = wasmtime::Module::new(&engine, wat)?;

    let mut linker = wasmtime::Linker::new(&engine);
    linker.func_wrap(
        "host",
        "hello",
        |caller: wasmtime::Caller<'_, u32>, param: i32| {
            println!("Got {} from WebAssembly", param);
            println!("My host state is: {}", caller.data());
        },
    )?;

    let mut store: wasmtime::Store<u32> = Store::new(&engine, 4);

    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;

    hello.call(&mut store, ())?;

    Ok(())
}
