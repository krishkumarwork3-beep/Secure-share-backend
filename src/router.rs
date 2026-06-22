use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{
    handler::{
        auth::auth_handler,
        file::file_handle,
        file_query::get_file_list_handler,
        user::users_handler,
    },
    middleware::auth,
    AppState,
};