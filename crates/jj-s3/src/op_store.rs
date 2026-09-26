use std::time::SystemTime;

use jj_lib::{
    object_id::{HexPrefix, PrefixResolution},
    op_store::{OpStore, OpStoreResult, Operation, OperationId, View, ViewId},
};

#[derive(Debug)]
pub(crate) struct S3OpStore {}

#[async_trait::async_trait]
impl OpStore for S3OpStore {
    fn name(&self) -> &str {
        todo!()
    }

    fn root_operation_id(&self) -> &OperationId {
        todo!()
    }

    async fn read_view(&self, id: &ViewId) -> OpStoreResult<View> {
        todo!()
    }

    async fn write_view(&self, contents: &View) -> OpStoreResult<ViewId> {
        todo!()
    }

    async fn read_operation(&self, id: &OperationId) -> OpStoreResult<Operation> {
        todo!()
    }

    async fn write_operation(&self, contents: &Operation) -> OpStoreResult<OperationId> {
        todo!()
    }

    async fn resolve_operation_id_prefix(
        &self,
        prefix: &HexPrefix,
    ) -> OpStoreResult<PrefixResolution<OperationId>> {
        todo!()
    }

    async fn gc(&self, head_ids: &[OperationId], keep_newer: SystemTime) -> OpStoreResult<()> {
        todo!()
    }
}
