use crate::model::{Body, Headers, Method, Url};

/// A HTTP request.
///
/// ```
/// use oxhttp::model::{Request, Method, Url, HeaderName, Body};
///
/// let mut request = Request::new(Method::POST, Url::parse("http://example.com:80/foo")?);
/// request.headers_mut().append(HeaderName::CONTENT_TYPE, "application/json".parse()?);
/// let request = request.with_body("{\"foo\": \"bar\"}");
///
/// assert_eq!(*request.method(), Method::POST);
/// assert_eq!(request.url().as_str(), "http://example.com/foo");
/// assert_eq!(&request.into_body().unwrap().to_vec()?, b"{\"foo\": \"bar\"}");
/// # Result::<_,Box<dyn std::error::Error>>::Ok(())
/// ```
#[derive(Debug)]
pub struct Request<'a> {
    method: Method,
    url: Url,
    headers: Headers,
    body: Option<Body<'a>>,
}

impl<'a> Request<'a> {
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body(&self) -> Option<&Body<'a>> {
        self.body.as_ref()
    }

    pub fn with_body(mut self, body: impl Into<Body<'a>>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn into_body(self) -> Option<Body<'a>> {
        self.body
    }
}
