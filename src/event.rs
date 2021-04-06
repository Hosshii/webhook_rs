// This file is copied from https://github.com/softprops/afterparty/blob/master/src/events.rs.in and edited.

// ************************************************************************
// Copyright (c) 2015-2016 Doug Tangren

// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
// ************************************************************************

use case::CaseExt;
pub fn patch_payload_json(event: &str, payload: &str) -> String {
    let mut patched_payload = "{\"".to_string();
    patched_payload.push_str(event.to_camel().as_ref());
    patched_payload.push_str("\":");
    patched_payload.push_str(payload);
    patched_payload.push_str("}");
    patched_payload
}

use serde::Deserialize;
#[derive(Debug, Deserialize, Default)]
pub struct Value {
    pub json: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub enum Event {
    CommitComment {
        action: String,
        comment: Comment,
        repository: Repository,
        sender: User,
    },
    Create {
        description: String,
        master_branch: String,
        pusher_type: String,
        #[serde(rename = "ref")]
        _ref: String,
        #[serde(rename = "ref_type")]
        ref_type: String,
        repository: Repository,
        sender: User,
    },
    Delete {
        pusher_type: String,
        #[serde(rename = "ref")]
        _ref: String,
        ref_type: String,
        repository: Repository,
        sender: User,
    },
    Deployment {
        deployment: Deployment,
        repository: Repository,
        sender: User,
    },
    DeploymentStatus {
        deployment: Deployment,
        deployment_status: DeploymentStatus,
        repository: Repository,
        sender: User,
    },
    Fork {
        forkee: Repository,
        repository: Repository,
        sender: User,
    },
    Gollum {
        pages: Vec<Pages>,
        repository: Repository,
        sender: User,
    },
    IssueComment {
        action: String,
        comment: IssueCommentComment,
        issue: Issue,
        repository: Repository,
        sender: User,
    },
    Issues {
        action: String,
        issue: Issue,
        repository: Repository,
        sender: User,
    },
    Member {
        action: String,
        member: User,
        repository: Repository,
        sender: User,
    },
    Membership {
        action: String,
        member: User,
        organization: Organization,
        scope: String,
        sender: User,
        team: Team,
    },
    PageBuild {
        build: PageBuild,
        id: u64,
        repository: Repository,
        sender: User,
    },
    Ping {
        hook: Hook,
        hook_id: u64,
        repository: Repository,
        sender: User,
        zen: String,
    },
    Public {
        repository: Repository,
        sender: User,
    },
    PullRequest {
        action: String,
        number: u64,
        pull_request: PullRequestDetails,
        repository: Repository,
        sender: User,
    },
    PullRequestReviewComment {
        action: String,
        comment: PullRequestReviewComment,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    Push {
        after: String,
        base_ref: Option<String>,
        before: String,
        commits: Vec<CommitStats>,
        compare: String,
        created: bool,
        deleted: bool,
        forced: bool,
        head_commit: Option<CommitStats>,
        pusher: UserRef, // note there aren't may fields here
        #[serde(rename = "ref")]
        _ref: String,
        repository: PushRepository,
        sender: User,
    },
    Release {
        action: String,
        release: Release,
        repository: Repository,
        sender: User,
    },
    Repository {
        action: String,
        organization: Organization,
        repository: Repository,
        sender: User,
    },
    Status {
        //branches: Vec<BranchRef>,
        commit: CommitRef,
        context: String,
        created_at: String,
        description: Option<String>,
        id: u64,
        name: String,
        repository: Repository,
        sender: User,
        sha: String,
        state: String,
        target_url: Option<String>,
        updated_at: String,
    },
    TeamAdd {
        organization: Organization,
        repository: Repository,
        sender: User,
        team: Team,
    },
    Watch {
        action: String,
        repository: Repository,
        sender: User,
    },
}

#[derive(Default, Debug, Deserialize)]
pub struct Commit {
    author: GitUser,
    committer: GitUser,
    message: String,
    tree: GitRef,
    url: String,
    comment_count: u64,
}

#[derive(Default, Debug, Deserialize)]
pub struct BranchRef {
    pub commit: GitRef,
    pub name: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct PageBuild {
    pub commit: String,
    pub created_at: String,
    pub duration: u64,
    pub error: Error,
    pub pusher: User,
    pub status: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Comment {
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub html_url: String,
    pub id: u64,
    pub line: Option<String>,
    pub path: Option<String>,
    pub position: Option<String>,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct CommitRef {
    pub author: User,
    pub comments_url: String,
    pub commit: Commit,
    pub committer: User,
    pub html_url: String,
    pub parents: Vec<GitRef>,
    pub sha: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Deployment {
    pub created_at: String,
    pub creator: User,
    pub description: Option<String>,
    pub environment: String,
    pub id: u64,
    pub payload: Value,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repository_url: String,
    pub sha: String,
    pub statuses_url: String,
    pub task: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct DeploymentStatus {
    pub created_at: String,
    pub creator: User,
    pub deployment_url: String,
    pub description: Option<String>,
    pub id: u64,
    pub repository_url: String,
    pub state: String,
    pub target_url: Option<String>,
    pub updated_at: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct CommitStats {
    pub added: Vec<String>,
    pub author: GitUser,
    pub committer: GitUser,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
    pub timestamp: String,
    pub tree_id: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Hook {
    pub active: bool,
    pub config: Config,
    pub created_at: String,
    pub events: Vec<String>,
    pub id: u64,
    pub last_response: LastResponse,
    pub name: String,
    pub ping_url: String,
    pub test_url: String,
    pub _type: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Issue {
    pub assignee: Option<Assignee>,
    pub assignees: Vec<Assignee>,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: u64,
    pub comments_url: String,
    pub created_at: String,
    pub events_url: String,
    pub html_url: String,
    pub id: u64,
    pub labels: Vec<Label>,
    pub labels_url: String,
    pub locked: bool,
    pub milestone: Option<MileStone>,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct IssueCommentComment {
    pub body: String,
    pub created_at: String,
    pub html_url: String,
    pub id: u64,
    pub issue_url: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct Organization {
    pub avatar_url: String,
    pub events_url: String,
    pub id: u64,
    pub login: String,
    pub members_url: String,
    pub public_members_url: String,
    pub repos_url: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Pages {
    pub action: String,
    pub html_url: String,
    pub page_name: String,
    pub sha: String,
    pub summary: Option<String>,
    pub title: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullRequestDetails {
    pub _links: PullRequestLinks,
    pub assignee: Option<Assignee>,
    pub base: PullSource,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: PullSource,
    pub html_url: String,
    pub id: u64,
    pub issue_url: String,
    pub locked: bool,
    pub merge_commit_sha: String,
    pub merged_at: Option<String>,
    pub milestone: Option<String>,
    pub number: u64,
    pub patch_url: String,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: String,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
    pub merged: bool,
    //mergeable": null,
    mergeable_state: String,
    //"merged_by": null,
    pub comments: u64,
    pub review_comments: u64,
    pub commits: u64,
    pub additions: u64,
    pub deletions: u64,
    pub changed_files: u64,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullRequest {
    pub _links: PullRequestLinks,
    pub assignee: Option<Assignee>,
    pub base: PullSource,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: PullSource,
    pub html_url: String,
    pub id: u64,
    pub issue_url: String,
    pub locked: bool,
    pub merge_commit_sha: String,
    pub merged_at: Option<String>,
    pub milestone: Option<String>,
    pub number: u64,
    pub patch_url: String,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: String,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullRequestReviewComment {
    #[serde(rename = "_links")]
    pub _links: PullRequestReviewCommentLinks,
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub diff_hunk: String,
    pub html_url: String,
    pub id: u64,
    pub original_commit_id: String,
    pub original_position: u64,
    pub path: String,
    pub position: u64,
    pub pull_request_url: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct Release {
    pub assets: Vec<String>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: u64,
    pub name: Option<String>,
    pub prerelease: bool,
    pub published_at: String,
    pub tag_name: String,
    pub tarball_url: String,
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct UserRef {
    pub name: String,
    pub email: Option<String>,
}

/// differs from Repository in owner type and some timestamp field types
#[derive(Default, Debug, Deserialize)]
pub struct PushRepository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: u64,
    pub default_branch: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_count: u64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: u64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub notifications_url: String,
    pub open_issues: u64,
    pub open_issues_count: u64,
    pub owner: UserRef,
    pub private: bool,
    pub pulls_url: String,
    pub pushed_at: u64,
    pub releases_url: String,
    pub size: u64,
    pub ssh_url: String,
    pub stargazers_count: u64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub watchers: u64,
    pub watchers_count: u64,
}

#[derive(Default, Debug, Deserialize)]
pub struct Repository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: String,
    pub default_branch: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub forks: u64,
    pub forks_count: u64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: u64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub notifications_url: String,
    pub open_issues: u64,
    pub open_issues_count: u64,
    pub owner: User,
    pub private: bool,
    pub pulls_url: String,
    pub pushed_at: String,
    pub releases_url: String,
    pub size: u64,
    pub ssh_url: String,
    pub stargazers_count: u64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub watchers: u64,
    pub watchers_count: u64,
}

#[derive(Default, Debug, Deserialize)]
pub struct Team {
    pub id: u64,
    pub members_url: String,
    pub name: String,
    pub permission: String,
    pub repositories_url: String,
    pub slug: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct GitUser {
    pub email: String,
    pub name: String,
    pub username: Option<String>,
    pub date: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub content_type: String,
    pub insecure_ssl: String,
    pub secret: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Error {
    pub message: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullSource {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct Label {
    pub color: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct LastResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    pub status: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub _self: Link,
    pub statuses: Link,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullRequestInnerBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullRequestInnerHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
pub struct PullRequestReviewCommentLinks {
    pub html: Link,
    pub pull_request: Link,
    #[serde(rename = "self")]
    pub _self: Link,
}

#[derive(Default, Debug, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: u64,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Link {
    pub href: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct GitRef {
    pub sha: String,
    pub url: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct MileStone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub state: String,
    pub title: String,
    pub description: String,
    pub creator: User,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub due_on: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Assignee {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    _type: String,
    site_admin: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_event_deserialize() {
        let events = ["commit_comment", "issues", "issue_comment"];

        for event in events.iter() {
            let filename = format!("data/{}.json", event);
            let content = fs::read_to_string(&filename).unwrap();
            let patched = patch_payload_json(event, &content);
            match serde_json::from_str::<Event>(&patched) {
                Ok(_) => (),
                Err(e) => {
                    println!("filename: {}", filename);
                    println!("{}", e);
                    panic!()
                }
            }
        }
    }
}
