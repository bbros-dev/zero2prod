use crate::steps::common::MyWorld;
pub use cucumber_rust::{async_trait, given, then, when, World, WorldInit};

#[given("a thing")]
async fn a_thing(world: &mut MyWorld) {
    world.foo = "elho".into();
    world.test_async_fn().await;
}
