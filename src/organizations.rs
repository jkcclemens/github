use github::GitHub;
use users::UserInfo;

#[derive(Debug)]
pub struct Organization<'a> {
  pub github: &'a GitHub,
  pub name: &'a str
}

pub type OrganizationInfo = UserInfo;
