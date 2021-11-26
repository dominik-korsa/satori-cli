pub struct Requests {
    client: reqwest::Client,
}

impl Requests {
    pub fn new() -> reqwest::Result<Requests> {
        Ok(Requests {
            client: reqwest::Client::builder().build()?,
        })
    }
}
