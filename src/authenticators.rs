use cassandra_proto::types::CBytes;

pub trait Authenticator: Clone + Send + Sync {
  fn get_auth_token(&self) -> CBytes;
  fn get_cassandra_name(&self) -> Option<&str>;
}

#[derive(Debug, Clone)]
pub struct PasswordAuthenticator {
  username: String,
  password: String,
}

impl PasswordAuthenticator {
  pub fn new<S: ToString>(username: S, password: S) -> PasswordAuthenticator {
    PasswordAuthenticator {
      username: username.to_string(),
      password: password.to_string(),
    }
  }
}

impl Authenticator for PasswordAuthenticator {
  fn get_auth_token(&self) -> CBytes {
    let mut token = vec![0];
    token.extend_from_slice(self.username.as_bytes());
    token.push(0);
    token.extend_from_slice(self.password.as_bytes());

    CBytes::new(token)
  }

  fn get_cassandra_name(&self) -> Option<&str> {
    Some("org.apache.cassandra.auth.PasswordAuthenticator")
  }
}

#[derive(Debug, Clone)]
pub struct NoneAuthenticator;

impl Authenticator for NoneAuthenticator {
  fn get_auth_token(&self) -> CBytes {
    CBytes::new(vec![0])
  }

  fn get_cassandra_name(&self) -> Option<&str> {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_password_authenticator_trait_impl() {
    let authenticator = PasswordAuthenticator::new("a", "a");
    let _ = authenticator_tester(Box::new(authenticator));
  }

  #[test]
  fn test_password_authenticator_new() {
    PasswordAuthenticator::new("foo", "bar");
  }

  #[test]
  fn test_password_authenticator_get_cassandra_name() {
    let auth = PasswordAuthenticator::new("foo", "bar");
    assert_eq!(
      auth.get_cassandra_name(),
      Some("org.apache.cassandra.auth.PasswordAuthenticator")
    );
  }

  #[test]
  fn test_password_authenticator_get_auth_token() {
    let auth = PasswordAuthenticator::new("foo", "bar");
    let mut expected_token = vec![0];
    expected_token.extend_from_slice("foo".as_bytes());
    expected_token.push(0);
    expected_token.extend_from_slice("bar".as_bytes());

    assert_eq!(auth.get_auth_token().into_plain().unwrap(), expected_token);
  }

  #[test]
  fn test_authenticator_none_get_cassandra_name() {
    let auth = NoneAuthenticator;
    assert_eq!(auth.get_cassandra_name(), None);
    assert_eq!(auth.get_auth_token().into_plain().unwrap(), vec![0]);
  }

  fn authenticator_tester<A: Authenticator>(_authenticator: Box<A>) {}
}
