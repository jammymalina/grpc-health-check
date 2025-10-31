pub mod pb {
    tonic::include_proto!("grpc.health.v1");
}

use clap::Parser;
use pb::HealthCheckRequest;
use pb::health_check_response::ServingStatus;
use pb::health_client::HealthClient;

const SERVING_STATUS_INT: i32 = ServingStatus::Serving as i32;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, env = "GRPC_HEALTH_CHECK_HOST")]
    host: String,
    #[arg(short, long, env = "GRPC_HEALTH_CHECK_PORT")]
    port: u64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut client = HealthClient::connect(format!("{}:{}", args.host, args.port)).await?;

    let health_request = HealthCheckRequest {
        service: "".to_string(),
    };
    let response = client.check(health_request).await?;
    let status = response.get_ref().status;

    match status {
        SERVING_STATUS_INT => Ok(()),
        _ => Err(anyhow::anyhow!("Unhealthy serving status: {}", status)),
    }
}
