use surf::{middleware::Middleware, middleware::Next, Client, Request, Response};

#[derive(Debug)]
pub(crate) struct ErrorForStatus;

#[surf::utils::async_trait]
impl Middleware for ErrorForStatus {
    async fn handle(
        &self,
        req: Request,
        client: Client,
        next: Next<'_>,
    ) -> Result<Response, surf::Error> {
        let url = req.url().clone();
        let res = next.run(req, client).await?;

        if !res.status().is_success() {
            // Return surf error.
            return Err(surf::Error::from_str(
                res.status(),
                format!("HTTPError ({}) {}", res.status(), url),
            ));
        }

        Ok(res)
    }
}