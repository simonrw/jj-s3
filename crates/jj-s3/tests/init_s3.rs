/*
async fn with_localstack<F>(f: F) -> &'static ContainerAsync<GenericImage> {
    CONTAINER
        .get_or_init(|| async {
            unsafe { std::env::set_var("AWS_PROFILE", "localstack") };

            let auth_token =
                std::env::var("LOCALSTACK_AUTH_TOKEN").expect("no auth token in environment");

            let container = GenericImage::new("localstack/localstack", "dev")
                .with_exposed_port(4566.tcp())
                .with_wait_for(WaitFor::Log(LogWaitStrategy::stdout("Ready")))
                .with_env_var("DEBUG", "1")
                .with_env_var("LOCALSTACK_AUTH_TOKEN", auth_token)
                .with_mount(Mount::bind_mount(
                    "/var/run/docker.sock",
                    "/var/run/docker.sock",
                ))
                .start()
                .await
                .expect("failed to start LocalStack");

            container
        })
        .await
}
*/

use aws_config::BehaviorVersion;

#[tokio::test]
async fn init_with_s3() {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    let res = client.list_buckets().send().await.unwrap();
    let buckets = res.buckets();
    assert!(buckets.is_empty());
}
