use std::fs;

use crate::fs::validate_path;
use axum::{extract::rejection::JsonRejection, Extension, Json};
use axum_auth::AuthBearer;
use byte_unit::Byte;
use homedisk_database::Database;
use homedisk_types::{
    config::types::Config,
    errors::{FsError, ServerError},
    fs::list::{FileInfo, Request, Response},
};

use crate::middleware::{find_user, validate_json, validate_jwt};

pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
    request: Result<Json<Request>, JsonRejection>,
) -> Result<Json<Response>, ServerError> {
    let Json(request) = validate_json::<Request>(request)?;
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    // validate the `path` can be used
    validate_path(&request.path)?;

    // search for a user by UUID from a token
    let user = find_user(db, token.claims.sub).await?;

    // directory where the file will be placed
    let path = format!(
        "{user_dir}/{req_dir}",
        user_dir = user.user_dir(&config.storage.path),
        req_dir = request.path
    );

    let paths = fs::read_dir(&path)
        .map_err(|err| ServerError::FsError(FsError::ReadDir(err.to_string())))?;

    let mut files = vec![];
    let mut dirs = vec![];

    for f in paths {
        let f = f.map_err(|err| ServerError::FsError(FsError::UnknowError(err.to_string())))?;
        let metadata = f
            .metadata()
            .map_err(|err| ServerError::FsError(FsError::UnknowError(err.to_string())))?;

        let name = f.path().display().to_string().replace(&path, "");
        let file_size = Byte::from_bytes(metadata.len().into()).get_appropriate_unit(true);

        if metadata.is_dir() {
            dirs.push(name)
        } else {
            files.push(FileInfo {
                name,
                size: file_size.to_string(),
            })
        }
    }

    Ok(Json(Response { files, dirs }))
}
