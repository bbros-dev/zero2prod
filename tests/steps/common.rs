use std::{cell::RefCell, convert::Infallible};

use cucumber_rust::{async_trait, given, when, World, WorldInit};

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

#[given("the sign-up service is monitored")]
async fn activate_monitor(world: &mut MyWorld) {
    world.foo = "monitor".into();
    world.test_async_fn().await;
}

// Write the code we wish we had:
//
#[when("the sign-up service is healthy")]
async fn signup_service_healthy(world: &mut MyWorld) {
    // Launch the health check as a background task.
    // No .await, no .expect
    spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    //
    // Use `cargo add reqwest --dev --vers 0.11` to add
    // it under `[dev-dependencies]` in Cargo.toml
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Writing the code we wish we had:
//
// No `.await` call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
//
async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    //
    // New dev dependency - let's add tokio to the party with
    // `cargo add tokio --dev --vers 1`
    let _ = tokio::spawn(server);
}
