use reqwest::header::HeaderValue;
use reqwest::Url;
use std::sync::{Mutex, MutexGuard, PoisonError};

pub struct MyCookieStore(Mutex<cookie_store::CookieStore>);

impl MyCookieStore {
    pub fn new(cookie_store: cookie_store::CookieStore) -> MyCookieStore {
        MyCookieStore(Mutex::new(cookie_store))
    }

    pub fn lock(
        &self,
    ) -> Result<
        MutexGuard<'_, cookie_store::CookieStore>,
        PoisonError<MutexGuard<'_, cookie_store::CookieStore>>,
    > {
        self.0.lock()
    }
}

impl reqwest::cookie::CookieStore for MyCookieStore {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let cookies = cookie_headers.filter_map(|val| {
            std::str::from_utf8(val.as_bytes())
                .map_err(cookie::ParseError::from)
                .and_then(cookie::Cookie::parse)
                .map(|c| {
                    let mut c = c.into_owned();
                    c.make_permanent();
                    c
                })
                .ok()
        });
        let mut cookie_store = self.0.lock().unwrap();
        cookie_store.store_response_cookies(cookies, url);
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let cookie_store = self.0.lock().unwrap();
        let s = cookie_store
            .get_request_values(url)
            .map(|(name, value)| format!("{}={}", name, value))
            .collect::<Vec<_>>()
            .join("; ");

        if s.is_empty() {
            return None;
        }

        HeaderValue::from_maybe_shared(bytes::Bytes::from(s)).ok()
    }
}
