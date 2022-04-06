/// `actix_tr::test` spares us from having to specify `#[test` attribute.
///
/// `cargo add actix-rt --dev --version 2` to add `actix-rt` in our Cargo.toml.
///
/// Inspect generated code with `cargo expand --test health_check` (<- test file name)

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // Bring in a request to perform
    // HTTP requests against our app.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// Without .await, no need fot `spawn_app` to be aysync.
/// Since we're running tests, no need to propagate errors,
/// we choose to panic and crash if if fails.
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch server as bkground task,
    // tokio::spawn returns a handle to the spawned future, but we don't use it here.
    // so it a non-binding let is used.
    let _ = tokio::spawn(server);
}
