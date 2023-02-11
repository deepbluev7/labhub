use headers::{Header, HeaderName, HeaderValue};

pub struct XGitHubEvent(pub String);

impl Header for XGitHubEvent {
    fn name() -> &'static HeaderName {
        static N: HeaderName = HeaderName::from_static("x-github-event");
        &N
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        Ok(XGitHubEvent(
            value
                .to_str()
                .or(Err(headers::Error::invalid()))?
                .to_owned(),
        ))
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::from_str(self.0.as_str());

        values.extend(value);
    }
}

pub struct XHubSignature(pub String);

impl Header for XHubSignature {
    fn name() -> &'static HeaderName {
        static N: HeaderName = HeaderName::from_static("x-hub-signature");
        &N
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        Ok(XHubSignature(
            value
                .to_str()
                .or(Err(headers::Error::invalid()))?
                .to_owned(),
        ))
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::from_str(self.0.as_str());

        values.extend(value);
    }
}

//#[derive(Debug)]
//pub enum RequestError {
//    BadCount,
//    Missing,
//}
//
//pub struct XGitHubEvent(pub String);
//
//impl<'a, 'r> FromRequest<'a, 'r> for XGitHubEvent {
//    type Error = RequestError;
//
//    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//        let events: Vec<_> = request.headers().get("X-GitHub-Event").collect();
//
//        match events.len() {
//            0 => Outcome::Failure((Status::BadRequest, RequestError::Missing)),
//            1 => Outcome::Success(XGitHubEvent(events[0].to_string())),
//            _ => Outcome::Failure((Status::BadRequest, RequestError::BadCount)),
//        }
//    }
//}
//
//pub struct XHubSignature(pub String);
//
//impl<'a, 'r> FromRequest<'a, 'r> for XHubSignature {
//    type Error = RequestError;
//
//    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//        let signatures: Vec<_> = request.headers().get("X-Hub-Signature").collect();
//
//        match signatures.len() {
//            0 => Outcome::Failure((Status::BadRequest, RequestError::Missing)),
//            1 => Outcome::Success(XHubSignature(signatures[0].to_string())),
//            _ => Outcome::Failure((Status::BadRequest, RequestError::BadCount)),
//        }
//    }
//}
