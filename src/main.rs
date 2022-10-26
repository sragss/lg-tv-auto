use lg_webos_client::client::*;
use lg_webos_client::command::Command;


#[tokio::main]
async fn main() {
	// env_logger::init();

	let config = WebOsClientConfig::new("ws://192.168.6.14:3000", Some("fa81fc9a6dc488ea1bfdd47648b76cc6".to_string()));
	let mut client = WebosClient::new(config).await.unwrap();
	println!("key: {:?}", client.key);
	let resp = client.send_command(Command::GetExternalInputList).await.unwrap();
	let response_payload = resp.payload.unwrap();
	println!("Response: {:?}", response_payload);

	let update = client.send_command(
		Command::SwitchInput("HDMI_1".to_string())).await.unwrap();
	let response_payload = update.payload.unwrap();
	println!("Response: {:?}", response_payload);
}
