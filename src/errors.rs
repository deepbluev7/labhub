use crate::api::github_signature;
use crate::commands;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::io;

#[derive(Debug)]
pub struct ResponseError {
    response: serde_json::Value,
}

#[derive(Debug)]
pub struct BadRequest {
    response: serde_json::Value,
}

#[derive(Debug)]
pub enum RequestErrorResult {
    BadRequest(BadRequest),
    ResponseError(ResponseError),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.response)).into_response()
    }
}

impl IntoResponse for BadRequest {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self.response)).into_response()
    }
}

impl IntoResponse for RequestErrorResult {
    fn into_response(self) -> Response {
        match self {
            RequestErrorResult::BadRequest(br) => br.into_response(),
            RequestErrorResult::ResponseError(re) => re.into_response(),
        }
    }
}

#[derive(Debug)]
pub struct GitError {
    pub message: String,
}

impl From<io::Error> for RequestErrorResult {
    fn from(error: io::Error) -> Self {
        RequestErrorResult::ResponseError {
            0: ResponseError {
                response: serde_json::json!({ "error": format!("{:?}", error) }),
            },
        }
    }
}

impl From<github_signature::SignatureError> for RequestErrorResult {
    fn from(error: github_signature::SignatureError) -> Self {
        RequestErrorResult::BadRequest {
            0: BadRequest {
                response: serde_json::json!({ "error": format!("{:?}", error) }),
            },
        }
    }
}

impl From<serde_json::error::Error> for RequestErrorResult {
    fn from(error: serde_json::error::Error) -> Self {
        RequestErrorResult::BadRequest {
            0: BadRequest {
                response: serde_json::json!({ "error": format!("{:?}", error) }),
            },
        }
    }
}

impl From<GitError> for RequestErrorResult {
    fn from(error: GitError) -> Self {
        RequestErrorResult::BadRequest {
            0: BadRequest {
                response: serde_json::json!({ "error": format!("{:?}", error) }),
            },
        }
    }
}

impl From<git2::Error> for GitError {
    fn from(error: git2::Error) -> Self {
        GitError {
            message: format!("Git error: {:?}", error.message()),
        }
    }
}

impl From<io::Error> for GitError {
    fn from(error: io::Error) -> Self {
        GitError {
            message: format!("Git error: {:?}", error),
        }
    }
}

impl From<serde_json::error::Error> for GitError {
    fn from(error: serde_json::error::Error) -> Self {
        GitError {
            message: format!("Github serde error: {:?}", error),
        }
    }
}

impl From<reqwest::Error> for GitError {
    fn from(error: reqwest::Error) -> Self {
        GitError {
            message: format!("Git request error: {:?}", error),
        }
    }
}

impl From<commands::CommandError> for GitError {
    fn from(error: commands::CommandError) -> Self {
        GitError {
            message: format!("Git command error: {:?}", error),
        }
    }
}
