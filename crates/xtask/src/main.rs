use clap::Parser;
use testcontainers::core::Mount;
use testcontainers::runners::SyncRunner;
use testcontainers::{
    GenericImage, ImageExt,
    core::{WaitFor, wait::LogWaitStrategy},
};

#[derive(Parser)]
enum Args {
    Test,
}

fn main() {
    let args = Args::parse();

    match args {
        Args::Test => {
            unsafe { std::env::set_var("AWS_PROFILE", "localstack") };

            let auth_token =
                std::env::var("LOCALSTACK_AUTH_TOKEN").expect("no auth token in environment");

            let container = GenericImage::new("localstack/localstack", "dev")
                .with_wait_for(WaitFor::Log(LogWaitStrategy::stdout("Ready")))
                .with_env_var("DEBUG", "1")
                .with_env_var("LOCALSTACK_AUTH_TOKEN", auth_token)
                .with_mount(Mount::bind_mount(
                    "/var/run/docker.sock",
                    "/var/run/docker.sock",
                ))
                .start()
                .expect("failed to start LocalStack");

            let localstack_port = container.get_host_port_ipv4(4566).unwrap();
            eprintln!("LocalStack listening on port {localstack_port}");

            unsafe {
                std::env::set_var(
                    "AWS_ENDPOINT_URL",
                    format!("http://localhost.localstack.cloud:{localstack_port}"),
                )
            };
            unsafe { std::env::set_var("LOCALSTACK_GATEWAY_PORT", format!("{localstack_port}")) };

            let st = std::process::Command::new("cargo")
                .args(&["test", "-p", "jj-s3"])
                .status()
                .unwrap();
            assert!(st.success());

            // drop should remove container
        }
    }
}
