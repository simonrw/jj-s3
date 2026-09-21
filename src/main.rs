mod backend;
mod op_heads_store;
mod op_store;

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

use crate::backend::S3Backend;

#[derive(clap::Parser, Clone, Debug)]
enum CustomCommand {
    /// Initialize a workspace using the S3 backend
    InitS3,
}

fn create_store_factories() -> StoreFactories {
    let mut store_factories = StoreFactories::empty();
    store_factories.add_backend(
        "s3",
        Box::new(|settings, store_path| Ok(Box::new(S3Backend::load(settings, store_path)?))),
    );
    store_factories
}

async fn run_custom_command(
    ui: &mut Ui,
    command_helper: &CommandHelper,
    command: CustomCommand,
) -> Result<(), CommandError> {
    match command {
        CustomCommand::InitS3 => {
            let wc_path = command_helper.cwd();
            let settings = command_helper.settings_for_new_workspace(ui, wc_path)?.0;

            Workspace::init_with_backend(
                &settings,
                wc_path,
                &|settings, store_path| Ok(Box::new(S3Backend::init(settings, store_path)?)),
                Signer::from_settings(&settings).map_err(WorkspaceInitError::SignInit)?,
            )
            .await?;
            Ok(())
        }
    }
}

fn main() -> std::process::ExitCode {
    CliRunner::init()
        .add_store_factories(create_store_factories())
        .add_subcommand(run_custom_command)
        .run()
        .into()
}
