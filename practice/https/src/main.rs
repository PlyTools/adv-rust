use serde_derive::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Post {
    // The field name must be alighed with the HTTPS Get Response
    userId: i32,
    id: i32,
    title: String,
    body: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response: Post = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json()
        .await?;
    println!("{:?}", response);
    Ok(())
}
