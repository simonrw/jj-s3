use std::{path::Path, pin::Pin, time::SystemTime};

use futures::{AsyncRead, stream::BoxStream};
use jj_cli::{
    cli_util::{CliRunner, CommandHelper},
    command_error::CommandError,
    ui::Ui,
};
use jj_lib::{
    backend::{
        Backend, BackendInitError, BackendLoadError, BackendResult, ChangeId, Commit, CommitId,
        CopyHistory, CopyId, CopyRecord, FileId, RelatedCopy, SigningFn, SymlinkId, Tree, TreeId,
    },
    index::Index,
    repo::StoreFactories,
    repo_path::{RepoPath, RepoPathBuf},
    settings::UserSettings,
    signing::Signer,
    workspace::{Workspace, WorkspaceInitError},
};

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

#[derive(Debug)]
struct S3Backend;

impl S3Backend {
    fn init(settings: &UserSettings, store_path: &Path) -> Result<Self, BackendInitError> {
        Ok(Self)
    }

    fn load(settings: &UserSettings, store_path: &Path) -> Result<Self, BackendLoadError> {
        Ok(Self)
    }
}

#[async_trait::async_trait]
impl Backend for S3Backend {
    fn name(&self) -> &str {
        "s3"
    }

    fn commit_id_length(&self) -> usize {
        32
    }

    fn change_id_length(&self) -> usize {
        8
    }

    fn root_commit_id(&self) -> &CommitId {
        todo!()
    }

    fn root_change_id(&self) -> &ChangeId {
        todo!()
    }

    fn empty_tree_id(&self) -> &TreeId {
        todo!()
    }

    fn concurrency(&self) -> usize {
        1
    }

    async fn read_file(
        &self,
        path: &RepoPath,
        id: &FileId,
    ) -> BackendResult<Pin<Box<dyn AsyncRead + Send>>> {
        todo!()
    }

    async fn write_file(
        &self,
        path: &RepoPath,
        contents: &mut (dyn AsyncRead + Send + Unpin),
    ) -> BackendResult<FileId> {
        todo!()
    }

    async fn read_symlink(&self, path: &RepoPath, id: &SymlinkId) -> BackendResult<String> {
        todo!()
    }

    async fn write_symlink(&self, path: &RepoPath, target: &str) -> BackendResult<SymlinkId> {
        todo!()
    }

    async fn read_copy(&self, id: &CopyId) -> BackendResult<CopyHistory> {
        todo!()
    }

    async fn write_copy(&self, copy: &CopyHistory) -> BackendResult<CopyId> {
        todo!()
    }

    async fn get_related_copies(&self, copy_id: &CopyId) -> BackendResult<Vec<RelatedCopy>> {
        todo!()
    }

    async fn read_tree(&self, path: &RepoPath, id: &TreeId) -> BackendResult<Tree> {
        todo!()
    }

    async fn write_tree(&self, path: &RepoPath, contents: &Tree) -> BackendResult<TreeId> {
        todo!()
    }

    async fn read_commit(&self, id: &CommitId) -> BackendResult<Commit> {
        todo!()
    }

    async fn write_commit(
        &self,
        contents: Commit,
        sign_with: Option<&mut SigningFn>,
    ) -> BackendResult<(CommitId, Commit)> {
        todo!()
    }

    fn get_copy_records(
        &self,
        paths: Option<&[RepoPathBuf]>,
        root: &CommitId,
        head: &CommitId,
    ) -> BackendResult<BoxStream<'_, BackendResult<CopyRecord>>> {
        todo!()
    }

    fn gc(&self, index: &dyn Index, keep_newer: SystemTime) -> BackendResult<()> {
        todo!()
    }
}

fn main() -> std::process::ExitCode {
    CliRunner::init()
        .add_store_factories(create_store_factories())
        .add_subcommand(run_custom_command)
        .run()
        .into()
}
