use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Service<Req, Res> {
    async fn execute(&self, command: Req) -> Res;
}
