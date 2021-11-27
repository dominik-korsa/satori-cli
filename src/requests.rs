use crate::config::{load_cookies, save_cookies};
use crate::my_cookie_store::MyCookieStore;
use crate::parser;
use heck::KebabCase;
use reqwest::redirect::Policy;
use std::path::Path;
use std::sync::Arc;

pub struct Requests {
    cookie_store: Arc<MyCookieStore>,
}

impl Requests {
    pub fn new() -> reqwest::Result<Requests> {
        let cookie_store = MyCookieStore::new(load_cookies());
        let cookie_store = Arc::new(cookie_store);
        let requests = Requests {
            cookie_store: cookie_store.clone(),
        };
        requests.refresh_token()?;
        Ok(requests)
    }

    pub fn get_client(
        &self,
        redirect: reqwest::redirect::Policy,
    ) -> reqwest::Result<reqwest::blocking::Client> {
        reqwest::blocking::Client::builder()
            .cookie_provider(self.cookie_store.clone())
            .redirect(redirect)
            .build()
    }

    fn refresh_token(&self) -> reqwest::Result<()> {
        let response = self
            .get_client(Policy::default())?
            .head("https://satori.tcs.uj.edu.pl/")
            .send()?;
        response.error_for_status()?;
        Ok(())
    }

    pub fn is_signed_in(&self) -> bool {
        self.cookie_store
            .lock()
            .unwrap()
            .get("satori.tcs.uj.edu.pl", "/", "satori_token")
            .map(|cookie| cookie.value())
            .unwrap_or("")
            != ""
    }

    pub fn sign_in(&self, username: &str, password: &str) -> reqwest::Result<bool> {
        let result = self
            .get_client(Policy::none())?
            .post("https://satori.tcs.uj.edu.pl/login")
            .form(&[("login", username), ("password", password)])
            .send()?;
        if result.status() == 200 {
            return Ok(false);
        }
        if result.status() == 302 {
            return Ok(true);
        }
        result.error_for_status()?;
        unreachable!();
    }

    pub fn sign_out(&self) -> reqwest::Result<()> {
        self.get_client(Policy::default())?
            .get("https://satori.tcs.uj.edu.pl/logout")
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn submit(
        &self,
        contest_id: &str,
        problem_id: &str,
        filename: &Path,
        content: &str,
    ) -> reqwest::Result<()> {
        let safe_filename = format!(
            "{}.{}",
            filename
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_kebab_case(),
            filename.extension().unwrap().to_str().unwrap()
        );
        let form = reqwest::blocking::multipart::Form::new()
            .text("problem", problem_id.to_string())
            .part(
                "codefile",
                reqwest::blocking::multipart::Part::text(content.to_string())
                    .file_name(safe_filename),
            );
        let result = self
            .get_client(Policy::none())?
            .post(format!(
                "https://satori.tcs.uj.edu.pl/contest/{}/submit",
                contest_id
            ))
            .multipart(form)
            .send()?;
        if result.status() == 200 {
            panic!(); // TODO: Add custom error type
        }
        if result.status() == 302 {
            return Ok(());
        }
        result.error_for_status()?;
        unreachable!();
    }

    pub fn get_latest_solution_id(
        &self,
        contest_id: &str,
        problem_id: &str,
    ) -> reqwest::Result<String> {
        let result = self
            .get_client(Policy::default())?
            .get(format!(
                "https://satori.tcs.uj.edu.pl/contest/{}/results?results_filter_problem={}",
                contest_id, problem_id,
            ))
            .send()?
            .error_for_status()?;
        Ok(parser::results::get_latest_solution_id(&result.text()?))
    }
}

impl Drop for Requests {
    fn drop(&mut self) {
        save_cookies(&mut self.cookie_store.lock().unwrap());
    }
}
