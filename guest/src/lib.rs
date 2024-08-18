use std::cell::RefCell;

use crate::exports::battalion::logic::types::*;


wit_bindgen::generate!({

    // the name of the world in the `*.wit` input file
    world: "plugin",
    path: "../wit/world.wit"
});

pub struct MyTotalizer {
    inner: RefCell<i64>
}

impl GuestTotalizer for MyTotalizer {
    fn new(i : i64) -> MyTotalizer {
        MyTotalizer { inner: RefCell::new(i) }
    }

    fn add(&self, i: i64) {
        let sum: i64 = *self.inner.borrow_mut() + i;
        battalion::logic::logging::log(battalion::logic::logging::Level::Info, "add");
        self.inner.replace(sum);
    }

    fn get(&self) -> i64 {
        *self.inner.borrow()
    }
}
struct MyPlugin;

impl Guest for MyPlugin {
    type Totalizer = MyTotalizer;
}

export!(MyPlugin);

