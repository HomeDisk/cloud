mod init;

use homedisk_utils::{config::Config, database::Database};

#[tokio::main]
async fn main() {
    init::init();

    let config = Config::parse().expect("parse configuration file");

    let db = Database::open("homedisk.db")
        .await
        .expect("open SQLite database");

    // change the type from Vec<String> to Vec<HeaderValue> so that the http server can correctly detect CORS hosts
    let origins = config
        .http
        .cors
        .iter()
        .map(|e| e.parse().expect("parse CORS host"))
        .collect();

    let host = format!(
        "{host}:{port}",
        host = config.http.host,
        port = config.http.port
    );

    homedisk_server::serve(host, origins, db, config)
        .await
        .expect("start http server");
}
