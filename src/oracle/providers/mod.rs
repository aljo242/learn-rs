pub trait Provider {
    async fn get_stuff(&self) -> Result<(), reqwest::Error>;
}

pub struct BitstampProvider {
    pub client: reqwest::Client,
}

impl Provider for BitstampProvider {
    async fn get_stuff(&self) -> Result<(), reqwest::Error> {
        let res = self
            .client
            .get("https://www.bitstamp.net/api/v2/ticker/")
            .body("")
            .send()
            .await?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!("Body:\n{}", res.text().await?);

        Ok(())
    }
}
