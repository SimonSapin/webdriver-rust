use rustc_serialize::json::{Json, ToJson, ParserError};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::convert::From;
use hyper::status::StatusCode;

#[derive(PartialEq, Debug)]
pub enum ErrorStatus {
    ElementNotSelectable,
    ElementNotVisible,
    InvalidArgument,
    InvalidCookieDomain,
    InvalidElementCoordinates,
    InvalidElementState,
    InvalidSelector,
    InvalidSessionId,
    JavascriptError,
    MoveTargetOutOfBounds,
    NoSuchAlert,
    NoSuchElement,
    NoSuchFrame,
    NoSuchWindow,
    ScriptTimeout,
    SessionNotCreated,
    StaleElementReference,
    Timeout,
    UnableToSetCookie,
    UnexpectedAlertOpen,
    UnknownError,
    UnknownPath,
    UnknownMethod,
    UnsupportedOperation,
}

pub type WebDriverResult<T> = Result<T, WebDriverError>;

#[derive(Debug)]
pub struct WebDriverError {
    pub status: ErrorStatus,
    pub message: String
}

impl fmt::Display for WebDriverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl WebDriverError {
    pub fn new(status: ErrorStatus, message: &str) -> WebDriverError {
        WebDriverError {
            status: status,
            message: message.to_string()
        }
    }

    pub fn status_code(&self) -> &'static str {
        match self.status {
            ErrorStatus::ElementNotSelectable => "element not selectable",
            ErrorStatus::ElementNotVisible => "element not visible",
            ErrorStatus::InvalidArgument => "invalid argument",
            ErrorStatus::InvalidCookieDomain => "invalid cookie domain",
            ErrorStatus::InvalidElementCoordinates => "invalid element coordinates",
            ErrorStatus::InvalidElementState => "invalid element state",
            ErrorStatus::InvalidSelector => "invalid selector",
            ErrorStatus::InvalidSessionId => "invalid session id",
            ErrorStatus::JavascriptError => "javascript error",
            ErrorStatus::MoveTargetOutOfBounds => "move target out of bounds",
            ErrorStatus::NoSuchAlert => "no such alert",
            ErrorStatus::NoSuchElement => "no such element",
            ErrorStatus::NoSuchFrame => "no such frame",
            ErrorStatus::NoSuchWindow => "no such window",
            ErrorStatus::ScriptTimeout => "script timeout",
            ErrorStatus::SessionNotCreated => "session not created",
            ErrorStatus::StaleElementReference => "stale element reference",
            ErrorStatus::Timeout => "timeout",
            ErrorStatus::UnableToSetCookie => "unable to set cookie",
            ErrorStatus::UnexpectedAlertOpen => "unexpected alert open",
            ErrorStatus::UnknownError => "unknown error",
            ErrorStatus::UnknownPath => "unknown command",
            ErrorStatus::UnknownMethod => "unknown command",
            ErrorStatus::UnsupportedOperation => "unsupported operation",
        }
    }

    pub fn http_status(&self) -> StatusCode {
        match self.status {
            ErrorStatus::ElementNotSelectable => StatusCode::BadRequest,
            ErrorStatus::ElementNotVisible => StatusCode::BadRequest,
            ErrorStatus::InvalidArgument => StatusCode::BadRequest,
            ErrorStatus::InvalidCookieDomain => StatusCode::BadRequest,
            ErrorStatus::InvalidElementCoordinates => StatusCode::BadRequest,
            ErrorStatus::InvalidElementState => StatusCode::BadRequest,
            ErrorStatus::InvalidSelector => StatusCode::BadRequest,
            ErrorStatus::InvalidSessionId => StatusCode::NotFound,
            ErrorStatus::JavascriptError => StatusCode::InternalServerError,
            ErrorStatus::MoveTargetOutOfBounds => StatusCode::InternalServerError,
            ErrorStatus::NoSuchAlert => StatusCode::BadRequest,
            ErrorStatus::NoSuchElement => StatusCode::NotFound,
            ErrorStatus::NoSuchFrame => StatusCode::BadRequest,
            ErrorStatus::NoSuchWindow => StatusCode::BadRequest,
            ErrorStatus::ScriptTimeout => StatusCode::RequestTimeout,
            ErrorStatus::SessionNotCreated => StatusCode::InternalServerError,
            ErrorStatus::StaleElementReference => StatusCode::BadRequest,
            ErrorStatus::Timeout => StatusCode::RequestTimeout,
            ErrorStatus::UnableToSetCookie => StatusCode::InternalServerError,
            ErrorStatus::UnexpectedAlertOpen => StatusCode::InternalServerError,
            ErrorStatus::UnknownError => StatusCode::InternalServerError,
            ErrorStatus::UnknownPath => StatusCode::NotFound,
            ErrorStatus::UnknownMethod => StatusCode::MethodNotAllowed,
            ErrorStatus::UnsupportedOperation => StatusCode::InternalServerError,
        }
    }

    pub fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl ToJson for WebDriverError {
    fn to_json(&self) -> Json {
        let mut data = BTreeMap::new();
        data.insert("status".to_string(), self.status_code().to_json());
        data.insert("message".to_string(), self.message.to_json());
        Json::Object(data)
    }
}

impl Error for WebDriverError {
    fn description(&self) -> &str {
        self.status_code()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<ParserError> for WebDriverError {
    fn from(err: ParserError) -> WebDriverError {
        let msg = format!("{:?}", err);
        WebDriverError::new(ErrorStatus::UnknownError, &msg[..])
    }
}
