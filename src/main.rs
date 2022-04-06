use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind
    // the address. Otherwise call .await on the server.
run()?.await
}
