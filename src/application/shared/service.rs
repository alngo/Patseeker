pub trait Service<Req, Res> {
    async fn execute(&self, command: Req) -> Res;
}
