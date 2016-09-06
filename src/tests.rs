extern crate dotenv;

use github::GitHub;
use repositories::{RepositoryCreationInfo, RepositoryOwner};
use self::dotenv::dotenv;
use std::{env, thread, time};

const SLEEP_TIME: u64 = 3000;

macro_rules! try_assert {
    ($expr:expr) => {{
      match $expr {
        Ok(x) => x,
        Err(e) => {
          println!("{}", e);
          assert!(false);
          return;
        }
      }
    }}
}

fn sleep_ms(ms: u64) {
  thread::sleep(time::Duration::from_millis(ms));
}

fn set_up_gh() -> GitHub {
  dotenv().ok();
  let user = env::var("RUST_GITHUB_USER").unwrap();
  let api_key = env::var("RUST_GITHUB_API_KEY").unwrap();
  GitHub::new(&user, &api_key)
}

#[test]
pub fn test_personal_repo() {
  let repo_name = "test";
  let gh = set_up_gh();
  let user = gh.get_user();
  let repo_info = try_assert!(user.create_repository(&RepositoryCreationInfo::new(repo_name)));
  assert!(repo_name == repo_info.name);
  let repo_info = try_assert!(user.get_repository(repo_name).info());
  assert!(repo_name == repo_info.name);
  sleep_ms(SLEEP_TIME);
  let delete_repo = gh.get_user().delete_repository(repo_name);
  assert!(delete_repo.is_ok());
}

#[test]
pub fn test_organization_repo() {
  let repo_name = "test";
  let gh = set_up_gh();
  let org_name = env::var("RUST_GITHUB_ORG_NAME").unwrap();
  let org = gh.get_organization(&org_name);
  let repo_info = try_assert!(org.create_repository(&RepositoryCreationInfo::new(repo_name)));
  assert!(repo_name == repo_info.name);
  sleep_ms(SLEEP_TIME);
  let repo_info = try_assert!(org.get_repository(repo_name).info());
  assert!(repo_name == repo_info.name);
  let delete_repo = org.delete_repository(repo_name);
  assert!(delete_repo.is_ok());
}
