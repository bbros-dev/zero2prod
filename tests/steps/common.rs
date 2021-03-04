use std::{cell::RefCell, convert::Infallible};

use cucumber_rust::{async_trait, given, then, when, World, WorldInit};

#[derive(WorldInit)]
pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    pub foo: String,
    bar: usize,
    some_value: RefCell<u8>,
}

impl MyWorld {
    pub async fn test_async_fn(&mut self) {
        *self.some_value.borrow_mut() = 123u8;
        self.bar = 123;
    }
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            foo: "wat".into(),
            bar: 0,
            some_value: RefCell::new(0),
        })
    }
}
