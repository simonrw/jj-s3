use std::{fs, path::Path, pin::Pin, time::SystemTime};

use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    config::Region,
    operation::get_object::GetObjectError,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
};
use futures::{AsyncRead, stream::BoxStream};
use jj_lib::{
    backend::{
        Backend, BackendError, BackendInitError, BackendLoadError, BackendResult, ChangeId, Commit,
        CommitId, CopyHistory, CopyId, CopyRecord, FileId, RelatedCopy, Signature, SigningFn,
        SymlinkId, Timestamp, Tree, TreeId, make_root_commit,
    },
    content_hash::ContentHash,
    index::Index,
    merge::Merge,
    repo_path::{RepoPath, RepoPathBuf},
    settings::UserSettings,
};
use sha1_checked::{Digest, Sha1};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct RemoteLocator {
    pub(crate) bucket_name: String,
    pub(crate) prefix: String,
}

impl RemoteLocator {
    pub(crate) fn new(bucket_name: impl Into<String>, prefix: impl Into<String>) -> Self {
        Self {
            bucket_name: bucket_name.into(),
            prefix: prefix.into(),
        }
    }

    pub(crate) fn load(store_path: &Path) -> Result<Self, BackendLoadError> {
        let bytes =
            fs::read(store_path.join("s3.json")).map_err(|error| BackendLoadError(error.into()))?;
        serde_json::from_slice(&bytes).map_err(|error| BackendLoadError(error.into()))
    }
}

#[derive(Debug)]
pub(crate) struct S3Backend {
    client: aws_sdk_s3::Client,
    bucket_name: String,
    prefix: String,

    // defaults for ids
    root_commit_id: CommitId,
    root_change_id: ChangeId,
    empty_tree_id: TreeId,
}

impl S3Backend {
    pub(crate) async fn init(
        backend_config: RemoteLocator,
        _settings: &UserSettings,
        store_path: &Path,
    ) -> Result<Self, BackendInitError> {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let client = aws_sdk_s3::Client::new(&config);

        let bytes =
            serde_json::to_vec(&backend_config).map_err(|error| BackendInitError(error.into()))?;
        fs::write(store_path.join("s3.json"), bytes)
            .map_err(|error| BackendInitError(error.into()))?;

        let root_tree_id = TreeId::from_hex("00000000");
        let root_change_id = ChangeId::from_hex("aaaaaaaa");

        // create the root commit
        let root_commit = make_root_commit(root_change_id.clone(), root_tree_id.clone());

        let this = Self {
            client,
            bucket_name: backend_config.bucket_name,
            prefix: backend_config.prefix,
            root_commit_id: CommitId::from_hex("00000000000000000000000000000000"),
            root_change_id: root_change_id,
            empty_tree_id: root_tree_id,
        };

        this.write_root_commit()
            .await
            .map_err(|e| BackendInitError(Box::new(e)))?;

        Ok(this)
    }

    pub(crate) async fn load(
        backend_config: RemoteLocator,
        _settings: &UserSettings,
        _store_path: &Path,
    ) -> Result<Self, BackendLoadError> {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let client = aws_sdk_s3::Client::new(&config);
        Ok(Self {
            client,
            bucket_name: backend_config.bucket_name,
            prefix: backend_config.prefix,
            root_commit_id: CommitId::from_hex("00000000000000000000000000000000"),
            root_change_id: ChangeId::from_hex("aaaaaaaa"),
            empty_tree_id: TreeId::from_hex("00000000"),
        })
    }

    async fn write_root_commit(&self) -> Result<(), BackendError> {
        let root_commit = make_root_commit(self.root_change_id.clone(), self.empty_tree_id.clone());
        let key = self.key_for_commit(&self.root_commit_id);

        let body = serde_json::to_vec(&root_commit).expect("serialize root commit");

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .body(body.into())
            .send()
            .await
            .map_err(|e| {
                // TODO: handle proper errors
                BackendError::WriteObject {
                    object_type: "commit",
                    source: Box::new(e),
                }
            })?;

        Ok(())
    }

    fn key_for_commit(&self, commit_id: &CommitId) -> String {
        format!("{}/objects/commits/{commit_id}", &self.prefix)
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
        &self.root_commit_id
    }

    fn root_change_id(&self) -> &ChangeId {
        &self.root_change_id
    }

    fn empty_tree_id(&self) -> &TreeId {
        &self.empty_tree_id
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
        let key = self.key_for_commit(id);

        let body = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .send()
            .await
            .map_err(|e| match e.as_service_error() {
                Some(GetObjectError::NoSuchKey(..)) => BackendError::ObjectNotFound {
                    object_type: "commit".to_string(),
                    hash: id.to_string(),
                    source: Box::new(e),
                },
                _ => BackendError::ReadObject {
                    object_type: "commit".to_string(),
                    hash: id.to_string(),
                    source: Box::new(e),
                },
            })?;

        let commit_bytes = body
            .body
            .collect()
            .await
            .expect("reading commit body")
            .to_vec();

        // let commit: Commit = serde_json::from_slice(&commit_bytes).expect("parsing commit");

        todo!()
    }

    async fn write_commit(
        &self,
        contents: Commit,
        sign_with: Option<&mut SigningFn>,
    ) -> BackendResult<(CommitId, Commit)> {
        let mut hasher = Sha1::new();
        contents.hash(&mut hasher);
        let commit_id = hasher.finalize();

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
