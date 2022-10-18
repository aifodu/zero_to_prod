use zero_to_prod_2::{run, utils::app_address};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_address = app_address();

    println!(
        "Server running on http://{}:{}",
        app_address.ip, app_address.port
    );

    run(app_address.listener)?.await
}
