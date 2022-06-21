mod create_dir;
mod delete;
mod download;
mod list;
mod upload;

/// Handle `/api/fs/*` requests
pub fn app() -> axum::Router {
    use axum::routing::{delete, get, post};

    axum::Router::new()
        .route("/list", post(list::handle))
        .route("/upload", post(upload::handle))
        .route("/delete", delete(delete::handle))
        .route("/download", get(download::handle))
        .route("/createdir", post(create_dir::handle))
}

pub fn validate_path(path: &str) -> Result<(), homedisk_types::errors::ServerError> {
    use homedisk_types::errors::{FsError, ServerError};

    // `path` can't contain `..`
    // to prevent attack attempts because by using a `..` you can access the previous folder
    if path.contains("..") {
        return Err(ServerError::FsError(FsError::ReadDirectory(
            "the `path` must not contain `..`".to_string(),
        )));
    }

    // `path` can't contain `~`
    // to prevent attack attempts because `~` can get up a directory on `$HOME`
    if path.contains('~') {
        return Err(ServerError::FsError(FsError::ReadDirectory(
            "the `path` must not contain `~`".to_string(),
        )));
    }

    Ok(())
}
