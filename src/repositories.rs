use github::GitHub;
use hyper::method::Method;
use organizations::{Organization, OrganizationInfo};
use serde_json;
use users::{User, UserInfo};

#[derive(Debug)]
pub struct Repository<'a, Owner> where Owner: 'a + RepositoryOwner<'a, Owner> {
  github: &'a GitHub,
  owner: &'a Owner,
  name: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryInfo {
  pub id: u64,
  pub owner: UserInfo,
  pub name: String,
  pub full_name: String,
  pub description: Option<String>,
  pub private: bool,
  pub fork: bool,
  pub url: String,
  pub html_url: String,
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
  pub deployments_url: String,
  pub downloads_url: String,
  pub events_url: String,
  pub forks_url: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  pub hooks_url: String,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  pub languages_url: String,
  pub merges_url: String,
  pub milestones_url: String,
  pub mirror_url: Option<String>,
  pub notifications_url: String,
  pub pulls_url: String,
  pub releases_url: String,
  pub ssh_url: String,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  pub trees_url: String,
  pub homepage: Option<String>,
  pub language: Option<String>,
  pub forks_count: u64,
  pub stargazers_count: u64,
  pub watchers_count: u64,
  pub size: u64,
  pub default_branch: String,
  pub open_issues_count: u64,
  pub has_issues: bool,
  pub has_wiki: bool,
  pub has_pages: bool,
  pub has_downloads: bool,
  pub pushed_at: String,
  pub created_at: String,
  pub updated_at: String,
  pub permissions: RepositoryPermissions,
  pub subscribers_count: Option<u64>,
  pub organization: Option<OrganizationInfo>,
  pub parent: Option<Box<RepositoryInfo>>,
  pub source: Option<Box<RepositoryInfo>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryPermissions {
  admin: bool,
  push: bool,
  pull: bool
}

#[derive(Debug, PartialEq, Serialize)]
pub struct RepositoryCreationInfo<'a> {
  name: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  homepage: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  private: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  has_issues: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  has_wiki: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  has_downloads: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  team_id: Option<isize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  auto_init: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  gitignore_template: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  license_template: Option<&'a str>
}

pub trait RepositoryOwner<'a, Owner> where Owner: 'a + RepositoryOwner<'a, Owner> {
  fn get_repository(&'a self, name: &'a str) -> Repository<'a, Owner>;

  fn create_repository(&self, info: &'a RepositoryCreationInfo<'a>) -> Result<RepositoryInfo, String>;

  fn delete_repository(&self, name: &str) -> Result<(), String>;

  fn get_name(&self) -> String;
}

trait RepositoryDeleter {
  fn internal_delete_repository(&self, github: &GitHub, owner: &str, name: &str) -> Result<(), String> {
    let url = format!("/repos/{owner}/{repo}", owner=owner, repo=name);
    let mut res = match github.make_authorized_request(Method::Delete, url.as_str()).send() {
      Ok(r) => r,
      Err(e) => return Err(e.to_string())
    };
    github.get_result(&mut res).map(|_| ())
  }
}

trait RepositoryCreator {
  fn internal_create_repository<'a>(&self, github: &GitHub, url: &str, info: &RepositoryCreationInfo<'a>) -> Result<RepositoryInfo, String> {
    let json = match serde_json::to_string(info) {
      Ok(j) => j,
      Err(e) => return Err(e.to_string())
    };
    let res = github.make_authorized_request(Method::Post, url)
      .body(json.as_str())
      .send();
    let mut res = match res {
      Ok(r) => r,
      Err(e) => return Err(e.to_string())
    };
    github.get_result(&mut res).and_then(|body| serde_json::from_str(&body).map_err(|e| e.to_string()))
  }
}

impl<'a> RepositoryCreationInfo<'a> {
  pub fn new(name: &'a str) -> Self {
    RepositoryCreationInfo {
      name: name,
      description: None,
      homepage: None,
      private: None,
      has_issues: None,
      has_wiki: None,
      has_downloads: None,
      team_id: None,
      auto_init: None,
      gitignore_template: None,
      license_template: None
    }
  }
}

impl<'a, Owner> Repository<'a, Owner> where Owner: 'a + RepositoryOwner<'a, Owner> {
  pub fn new(github: &'a GitHub, owner: &'a Owner, name: &'a str) -> Self {
    Repository {
      github: github,
      owner: owner,
      name: name
    }
  }

  pub fn info(&self) -> Result<RepositoryInfo, String> {
    let url = format!("/repos/{owner}/{repo}", owner=self.owner.get_name(), repo=self.name);
    let mut res = match self.github.make_authorized_request(Method::Get, &url).send() {
      Ok(r) => r,
      Err(e) => return Err(e.to_string())
    };
    self.github.get_result(&mut res).and_then(|body| serde_json::from_str(&body).map_err(|e| e.to_string()))
  }
}

impl<'a> RepositoryCreator for User<'a> {}
impl<'a> RepositoryDeleter for User<'a> {}

impl<'a> RepositoryCreator for Organization<'a> {}
impl<'a> RepositoryDeleter for Organization<'a> {}

impl<'a> RepositoryOwner<'a, User<'a>> for User<'a> {
  fn get_repository(&'a self, name: &'a str) -> Repository<'a, User<'a>> {
    Repository::new(self.github, self, name)
  }

  fn create_repository(&self, info: &'a RepositoryCreationInfo<'a>) -> Result<RepositoryInfo, String> {
    self.internal_create_repository(self.github, "/user/repos", info)
  }

  fn delete_repository(&self, name: &str) -> Result<(), String> {
    self.internal_delete_repository(self.github, &self.github.username, name)
  }

  fn get_name(&self) -> String {
    self.github.username.to_owned()
  }
}

impl<'a> RepositoryOwner<'a, Organization<'a>> for Organization<'a> {
  fn get_repository(&'a self, name: &'a str) -> Repository<'a, Organization<'a>> {
    Repository::new(self.github, self, name)
  }

  fn create_repository(&self, info: &'a RepositoryCreationInfo<'a>) -> Result<RepositoryInfo, String> {
    let url = format!("/orgs/{organization}/repos", organization=self.name);
    self.internal_create_repository(self.github, &url, info)
  }

  fn delete_repository(&self, name: &str) -> Result<(), String> {
    self.internal_delete_repository(self.github, self.name, name)
  }

  fn get_name(&self) -> String {
    self.name.to_owned()
  }
}
