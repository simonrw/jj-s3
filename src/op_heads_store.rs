use jj_lib::{
    op_heads_store::{OpHeadsStore, OpHeadsStoreError, OpHeadsStoreLock},
    op_store::OperationId,
};

#[derive(Debug)]
pub(crate) struct S3OpHeadsStore {}

#[async_trait::async_trait]
impl OpHeadsStore for S3OpHeadsStore {
    fn name(&self) -> &str {
        todo!()
    }

    async fn update_op_heads(
        &self,
        old_ids: &[OperationId],
        new_id: &OperationId,
    ) -> Result<(), OpHeadsStoreError> {
        todo!()
    }

    async fn get_op_heads(&self) -> Result<Vec<OperationId>, OpHeadsStoreError> {
        todo!()
    }

    async fn lock(&self) -> Result<Box<dyn OpHeadsStoreLock + '_>, OpHeadsStoreError> {
        todo!()
    }
}
