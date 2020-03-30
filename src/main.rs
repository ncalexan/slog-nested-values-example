use erased_serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
use slog_json;

use slog::Drain; // For slog::Fuse.

#[derive(Clone, Serialize)]
struct Wrapper<T>(T);

#[derive(Clone, Serialize)]
struct ObjectType {
    x: i64,
    y: String,
}

impl<T> slog::SerdeValue for Wrapper<T>
where T: serde::Serialize + Clone + Send + 'static
{
  fn as_serde(&self) -> &dyn erased_serde::Serialize {
      self
  }

  fn to_sendable(&self) -> Box<dyn slog::SerdeValue + Send + 'static> {
      Box::new(self.clone())
  }
}

impl<T> slog::Value for Wrapper<T>
where T: serde::Serialize + Clone + Send + 'static
{
    fn serialize(&self, _: &slog::Record<'_>, key: slog::Key, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde(key, self)
    }
}

fn main() {
    let drain = 
        std::sync::Mutex::new(
            slog_json::Json::default(std::io::stderr())
        ).map(slog::Fuse);

    let logger = slog::Logger::root(drain, slog_o!("version" => env!("CARGO_PKG_VERSION")));

    slog_info!(logger, "JSON list"; "list" => Wrapper(vec![123, 456]));
    slog_info!(logger, "JSON object"; "object" => Wrapper(ObjectType { x: 1, y: "test".to_string() }));
}
