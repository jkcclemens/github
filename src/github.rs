use hyper::client::RequestBuilder;
use hyper::client::response::Response;
use hyper::Client;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper;
use organizations::Organization;
use serde_json;
use std::io::Read;
use users::User;

const GITHUB_API_BASE: &'static str = "https://api.github.com";

#[derive(Debug)]
pub struct GitHub {
  pub username: String,
  pub api_key: String,
  pub client: Client
}

#[derive(Debug, Deserialize)]
struct GitHubErrorResult {
  message: String,
  #[serde(default)]
  errors: Vec<GitHubErrorPart>
}

#[derive(Debug, Deserialize)]
struct GitHubErrorPart {
  field: String,
  code: String,
  #[serde(default)]
  message: String
}

impl GitHub {
  pub fn new(username: &str, api_key: &str) -> Self {
    GitHub {
      username: username.to_owned(),
      api_key: api_key.to_owned(),
      client: Client::new()
    }
  }

  pub fn get_user<'a>(&'a self) -> User<'a> {
    User {
      github: self
    }
  }

  pub fn get_organization<'a>(&'a self, name: &'a str) -> Organization<'a> {
    Organization {
      github: self,
      name: name
    }
  }

  pub fn make_authorized_request<'b>(&'b self, method: Method, url: &'b str) -> RequestBuilder<'b> {
    let url = format!("{}{}", GITHUB_API_BASE, url);
    self.client
      .request(method, url.as_str())
      .header(hyper::header::Authorization(
        hyper::header::Basic {
          username: self.username.to_owned(),
          password: Some(self.api_key.to_owned())
        }
      ))
      .header(hyper::header::UserAgent("rust github/0.1.0".to_owned()))
  }

  pub fn get_result(&self, response: &mut Response) -> Result<String, String> {
    let mut body = String::new();
    match response.read_to_string(&mut body) {
      Ok(_) => {},
      Err(e) => return Err(e.to_string())
    }
    if response.status.class().default_code() == StatusCode::Ok {
      Ok(body)
    } else {
      match serde_json::from_str::<GitHubErrorResult>(&body) {
        Ok(r) => Err(r.message),
        Err(e) => Err(format!("could not convert github's response into an error: {}\n\n{}", e, body))
      }
    }
  }
}
