mod oracle;

use error_chain::error_chain;
use crate::oracle::providers::Provider;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let client = reqwest::Client::new();
    let prov = oracle::providers::BitstampProvider { client };

    prov.get_stuff().await?;

    Ok(())
}
