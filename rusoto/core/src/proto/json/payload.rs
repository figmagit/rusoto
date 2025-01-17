use bytes::Bytes;
use http::StatusCode;
use log::*;
use serde::de::DeserializeOwned;
use serde_json::from_slice;

use super::super::super::request::BufferedHttpResponse;
use super::super::super::RusotoError;

pub struct ResponsePayload {
    body: Bytes,
    status: StatusCode,
}

impl ResponsePayload {
    pub fn new(res: &BufferedHttpResponse) -> Self {
        let mut body = res.body.clone();

        // `serde-json` serializes field-less structs as "null", but AWS returns
        // "{}" for a field-less response, so we must check for this result
        // and convert it if necessary.
        if body.is_empty() || body.as_ref() == b"null" {
            body = Bytes::from_static(b"{}");
        }

        debug!("Response body: {:?}", body);
        debug!("Response status: {}", res.status);

        Self { body, status: res.status }
    }

    pub fn deserialize<T: DeserializeOwned, E>(&self) -> Result<T, RusotoError<E>> {
        match from_slice(&self.body) {
            Ok(t) => Ok(t),
            Err(e) => {
                error!("Response body: {:?}", self.body);
                error!("Response status: {:?}", self.status);
                Err(e.into())
            }
        }
    }
}
