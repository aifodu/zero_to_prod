// use std::net::TcpListener;

// use zero_to_prod_2::configuration::get_configuration;
use zero_to_prod_2::startup::run;
use zero_to_prod_2::utils::app_address;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_address = app_address();

    // Remove the following lines and get app_address automatically as before

    // let configuration = get_configuration().expect("Failed to read configuration.");
    // // We have removed the hard-coded `8000` - it's now coming from our settings!
    // let address = format!("127.0.0.1:{}", configuration.application_port.to_string());
    // let listener = TcpListener::bind(address)?;

    println!(
        "Server running on http://{}:{}",
        app_address.ip, app_address.port
    );

    // println!("Server running on http://{}", address);

    run(app_address.listener)?.await
    // run(listener)?.await
}
