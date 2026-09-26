mod backend;
mod op_heads_store;
mod op_store;

use color_eyre::eyre::{self, Context};
use jj_cli::{
    cli_util::{CliRunner, CommandHelper},
    command_error::CommandError,
    ui::Ui,
};
use jj_lib::{
    repo::StoreFactories,
    signing::Signer,
    workspace::{Workspace, WorkspaceInitError},
};
use tokio::runtime::Handle;

use crate::backend::{RemoteLocator, S3Backend};

#[derive(clap::Parser, Clone, Debug)]
enum CustomCommand {
    /// Initialize a workspace using the S3 backend
    /*
    *    jj-s3 init-s3 \
     --bucket mybucket \
     --prefix repos/example \
    */
    InitS3 {
        #[arg(short, long)]
        bucket: String,

        #[arg(short, long)]
        prefix: String,
    },
}

fn create_store_factories() -> StoreFactories {
    let mut store_factories = StoreFactories::empty();
    store_factories.add_backend(
        "s3",
        Box::new(|settings, store_path| {
            let backend_config = RemoteLocator::load(store_path)?;
            let handle = Handle::current();

            Ok(Box::new(handle.block_on(async {
                S3Backend::load(backend_config, settings, store_path).await
            })?))
        }),
    );
    store_factories
}

async fn run_custom_command(
    ui: &mut Ui,
    command_helper: &CommandHelper,
    command: CustomCommand,
) -> Result<(), CommandError> {
    match command {
        CustomCommand::InitS3 { bucket, prefix } => {
            let wc_path = command_helper.cwd();
            let settings = command_helper.settings_for_new_workspace(ui, wc_path)?.0;

            let sanitised_prefix = sanitise_prefix(prefix).map_err(|e| {
                CommandError::with_message(
                    jj_cli::command_error::CommandErrorKind::User,
                    "invalid prefix specified",
                    e,
                )
            })?;
            let backend_config = RemoteLocator::new(bucket, sanitised_prefix);

            Workspace::init_with_backend(
                &settings,
                wc_path,
                &|settings, store_path| {
                    let handle = Handle::current();
                    let backend_config = backend_config.clone();

                    Ok(Box::new(handle.block_on(async {
                        S3Backend::init(backend_config, settings, store_path).await
                    })?))
                },
                Signer::from_settings(&settings).map_err(WorkspaceInitError::SignInit)?,
            )
            .await?;
            Ok(())
        }
    }
}

fn sanitise_prefix(prefix: String) -> eyre::Result<String> {
    let prefix = prefix.trim_end_matches('/').to_string();
    eyre::ensure!(
        prefix.is_empty() || prefix.chars().all(|c| c.is_whitespace()),
        "empty prefix supplied"
    );
    Ok(prefix)
}

fn main() -> std::process::ExitCode {
    if let Err(e) = color_eyre::install() {
        eprintln!("error installing error handling: {e}");
        return std::process::ExitCode::FAILURE;
    }
    // jj drives futures with Pollster, which provides no I/O or timer reactor.
    // Keep Tokio's worker threads running for AWS requests throughout the CLI.
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("Failed to start Tokio runtime: {error}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let _guard = runtime.enter();

    CliRunner::init()
        .add_store_factories(create_store_factories())
        .add_subcommand(run_custom_command)
        .run()
        .into()
}
