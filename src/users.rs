use github::GitHub;

#[derive(Debug)]
pub struct User<'a> {
  pub github: &'a GitHub
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
  pub login: String,
  pub id: u64,
  pub avatar_url: String,
  pub gravatar_id: String,
  pub url: String,
  pub html_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  pub starred_url: String,
  pub subscriptions_url: String,
  pub organizations_url: String,
  pub repos_url: String,
  pub events_url: String,
  pub received_events_url: String,
  #[serde(rename = "type")]
  pub type_key: String,
  pub site_admin: bool
}
