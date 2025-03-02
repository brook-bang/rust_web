use std::{collections::HashMap, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp);
    Ok(())
}

// {
//     "origin": "1.30.3.223",
// }
