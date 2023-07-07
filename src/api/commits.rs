//! The commit API.
mod compare_commit;
mod create_comment;

pub use self::create_comment::CreateCommentBuilder;
// pub use self::create_comment::
use crate::{models, Octocrab};

pub struct CommitHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> CommitHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    // pub fn create(&self, title: impl Into<String>) -> create::CreateIssueBuilder<'_, '_> {
    //     create::CreateIssueBuilder::new(self, title.into())
    // }

    pub fn compare(
        &self,
        base: impl Into<String>,
        head: impl Into<String>,
    ) -> compare_commit::CompareCommitsBuilder<'_, '_> {
        compare_commit::CompareCommitsBuilder::new(self, base.into(), head.into())
    }

    pub fn create_comment(
        &self,
        sha: impl Into<String>,
        body: impl Into<String>,
    ) -> create_comment::CreateCommentBuilder<'_, '_> {
        create_comment::CreateCommentBuilder::new(self, sha.into(), body.into())
    }
}
