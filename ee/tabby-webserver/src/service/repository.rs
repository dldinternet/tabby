use anyhow::Result;
use async_trait::async_trait;
use juniper::ID;
use tabby_db::{DbConn, SQLXRepositoryService};

use super::graphql_pagination_to_filter;
use crate::schema::repository::{Repository, RepositoryService};

#[async_trait]
impl RepositoryService for DbConn {
    async fn list_repositories(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<usize>,
        last: Option<usize>,
    ) -> Result<Vec<Repository>> {
        let (limit, skip_id, backwards) = graphql_pagination_to_filter(after, before, first, last)?;
        let repositories = self
            .list_repositories_with_filter(limit, skip_id, backwards)
            .await?;
        Ok(repositories.into_iter().map(Into::into).collect())
    }

    async fn create_repository(&self, name: String, git_url: String) -> Result<ID> {
        self.create_repository(name, git_url)
            .await
            .map(|i| ID::new(i.to_string()))
    }

    async fn delete_repository(&self, id: ID) -> Result<bool> {
        self.delete_repository(id.parse()?).await
    }
}

#[async_trait]
impl RepositoryService for SQLXRepositoryService {
    async fn list_repositories(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<usize>,
        last: Option<usize>,
    ) -> Result<Vec<Repository>> {
        let (limit, skip_id, backwards) = graphql_pagination_to_filter(after, before, first, last)?;
        let repositories = self
            .list_repositories_with_filter(limit, skip_id, backwards)
            .await?;
        Ok(repositories.into_iter().map(Into::into).collect())
    }

    async fn create_repository(&self, name: String, git_url: String) -> Result<ID> {
        Ok(self
            .create_repository(name, git_url)
            .await
            .map(|i| ID::new(i.to_string()))?)
    }

    async fn delete_repository(&self, id: ID) -> Result<bool> {
        self.delete_repository(id.parse()?).await?;
        Ok(true)
    }
}