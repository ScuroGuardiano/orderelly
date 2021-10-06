pub struct Service {
  pub name: String,
  pub path: String,
  pub thumbnail_url: Option<String>
}

impl Service {
  pub fn new(name: &str, path: &str, thumbnail_url: Option<&str>) -> Self {
    let mut thumb: Option<String> = None;
    if let Some(thumbnail_url) = thumbnail_url {
      thumb = Some(String::from(thumbnail_url));
    }

    Service {
      name: String::from(name),
      path: String::from(path),
      thumbnail_url: thumb,
    }
  }
}

/// Returns all of publicly listed services
pub fn get_services() -> Vec<Service> {
  // Insert your services here if you want to list em.
  vec![
    Service::new("Demo", "/demo", None)
  ]
}
