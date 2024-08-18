use std::time::Instant;
use wasmtime::component::*;
use wasmtime::Store;
use crate::battalion::logic::logging::{Host, Level};

bindgen!({
    world: "plugin",
    path: "../wit/world.wit",
});

struct MyHost;

impl Host for MyHost {
    fn log(&mut self, _level: Level, message: String) -> () {
        println!("{message}");
    }
}

fn main() -> wasmtime::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let engine = wasmtime::Engine::default();
    let component = Component::from_file(&engine, &args[1])?;
    let mut linker = Linker::new(&engine);
    Plugin::add_to_linker(&mut linker, |state: &mut MyHost| state)?;
    let mut store = Store::new(
        &engine,
        MyHost
    );

    println!("instantiating");

    let plugin = Plugin::instantiate(&mut store, &component, &linker)?;

    println!("starting");

    let start = Instant::now();
    let total = plugin.battalion_logic_types().totalizer().call_constructor(&mut store, 0)?;

    for i in 0..1_000 {
        plugin.battalion_logic_types().totalizer().call_add(&mut store, total, i)?;
    }

    let total = plugin.battalion_logic_types().totalizer().call_get  (&mut store, total)?;
    let elapsed = start.elapsed();


    println!("total is {total} in {elapsed:?}");


    Ok(())
}
