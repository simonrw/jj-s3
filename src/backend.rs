use std::{path::Path, pin::Pin, time::SystemTime};

use aws_config::BehaviorVersion;
use futures::{AsyncRead, stream::BoxStream};
use jj_lib::{
    backend::{
        Backend, BackendInitError, BackendLoadError, BackendResult, ChangeId, Commit, CommitId,
        CopyHistory, CopyId, CopyRecord, FileId, RelatedCopy, SigningFn, SymlinkId, Tree, TreeId,
    },
    index::Index,
    repo_path::{RepoPath, RepoPathBuf},
    settings::UserSettings,
};

static BUCKET_NAME: &'static str = "mybucket";

#[derive(Debug)]
pub(crate) struct S3Backend {
    client: aws_sdk_s3::Client,
}

impl S3Backend {
    pub(crate) async fn init(
        settings: &UserSettings,
        store_path: &Path,
    ) -> Result<Self, BackendInitError> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = aws_sdk_s3::Client::new(&config);

        // only create the bucket on init
        client
            .create_bucket()
            .bucket(BUCKET_NAME)
            .send()
            .await
            .map_err(|e| BackendInitError(format!("error creating bucket: {e}").into()))?;

        Ok(Self { client })
    }

    pub(crate) async fn load(
        settings: &UserSettings,
        store_path: &Path,
    ) -> Result<Self, BackendLoadError> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = aws_sdk_s3::Client::new(&config);
        Ok(Self { client })
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
