use base64::prelude::*;
use clap::Parser;
use hmac::Mac;
use reqwest::header;

macro_rules! print_if_dbg {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            dbg!($($arg)*);
        }
    };
}

// Struct Args for Clap
// usage: rst_if --token|-t <token> --secret|-s <secret> list
// usage: rst_if --token|-t <token> --secret|-s <secret> status --device_id|-i <device_id>
// usage: rst_if --token|-t <token> --secret|-s <secret> control --device_id|-i <device_id> --cmd|-c <cmd> --param|-p <param:json>
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about ="SwitchBot API CLI tool\nFor detailed information about commands and parameters, please refer to https://github.com/OpenWonderLabs/SwitchBotAPI.",
    long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
    #[arg(short = 't', long)]
    token: String,
    #[arg(short = 's', long)]
    secret: String,
}

#[derive(Parser, Debug)]
enum Action {
    /// List all devices
    List,
    /// Get the status of a device
    Status {
        /// Device ID
        #[arg(short = 'i', long)]
        device_id: String,
    },
    /// Send a command to a device
    Control {
        /// Device ID
        #[arg(short = 'i', long)]
        device_id: String,
        /// Command to send
        #[arg(short = 'c', long)]
        cmd: String,
        /// Command parameters in JSON format
        #[arg(short = 'p', long)]
        param: String,
    },
}

fn get_sign(token: &str, t: i64, nonce: &str, secret: &str) -> anyhow::Result<String> {
    let data = format!("{}{}{}", token, t, nonce);
    print_if_dbg!(&data);
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.to_string().as_bytes())?;
    mac.update(data.as_bytes());
    let result = BASE64_STANDARD.encode(mac.finalize().into_bytes());
    print_if_dbg!(&result);
    Ok(result)
}

fn gen_list_uri(root_uri: &str) -> String {
    format!("{}/", root_uri)
}
fn gen_status_uri(root_uri: &str, device_id: &String) -> String {
    format!("{}/{}/status", root_uri, device_id)
}
fn gent_control_uri(root_uri: &str, device_id: &String) -> String {
    format!("{}/{}/commands", root_uri, device_id)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let root_uri = "https://api.switch-bot.com/v1.1/devices";
    let t = chrono::Utc::now().timestamp_millis();
    let nonce = uuid::Uuid::new_v4().to_string();
    let sign = get_sign(&args.token, t, &nonce, &args.secret)?;

    let uri = match &args.action {
        Action::List => gen_list_uri(root_uri),
        Action::Status { device_id } => gen_status_uri(root_uri, &device_id),
        Action::Control {
            device_id,
            cmd: _,
            param: _,
        } => gent_control_uri(root_uri, &device_id),
    };
    print_if_dbg!(&uri);

    let client = reqwest::Client::builder()
        .default_headers(header::HeaderMap::from_iter(
            vec![
                (header::AUTHORIZATION, args.token.parse().unwrap()),
                (header::CONTENT_TYPE, "application/json".parse().unwrap()),
                (
                    header::HeaderName::from_static("charset"),
                    "utf8".parse().unwrap(),
                ),
                (
                    header::HeaderName::from_static("t"),
                    t.to_string().parse().unwrap(),
                ),
                (
                    header::HeaderName::from_static("sign"),
                    sign.parse().unwrap(),
                ),
                (
                    header::HeaderName::from_static("nonce"),
                    nonce.parse().unwrap(),
                ),
            ]
            .into_iter(),
        ))
        .build()?;
    let response = match &args.action {
        Action::List | Action::Status { .. } => client.get(uri),
        Action::Control {
            device_id: _,
            cmd,
            param,
        } => client.post(uri).json(&serde_json::json!({
            "command": cmd,
            "parameter": param,
            "commandType": "command",
        })),
    }
    .send()
    .await?
    .text()
    .await?;

    print!("{response}");

    Ok(())
}
