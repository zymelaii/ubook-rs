use super::*;
use async_trait::async_trait;

#[async_trait]
impl crate::api::AuthorAPI for BoluobaoHost {
    async fn query_author_info(&mut self, author_id: Id) -> crate::Result<AuthorInfo> {
        let _ = author_id;
        todo!()
    }

    async fn query_work_list(&mut self, author_id: Id) -> crate::Result<Vec<WorkRef>> {
        let _ = author_id;
        todo!()
    }
}
