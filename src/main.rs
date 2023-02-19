use reqwest::header;
use header::HeaderMap;
use reqwest::Client;

struct ClientInterface {
    key: String,
    url: String,
    client: Client,
    headers: HeaderMap
}

impl ClientInterface {
    pub fn new() -> Self {
        Self {   
                key: String::from("yourkeyhere"), 
                url: String::from("https://api.openai.com/v1/models"), 
                client: Client::new(), 
                headers: HeaderMap::new()
        }
    }
    pub fn insert_headers(&mut self) {
        self.headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", self.key)).unwrap());
    }
    pub async fn get_models(&self) -> Result<String, reqwest::Error> {
        let response = self.client.get(&self.url)
            .headers(self.headers.clone())
            .send()
            .await?;
        let output = response.text().await?;
        Ok(output)
    }
}

#[tokio::main]
async fn main() {
    let mut ci = ClientInterface::new();
    ci.insert_headers();
    let models = ci.get_models().await.unwrap();

    println!("{}", models);
}