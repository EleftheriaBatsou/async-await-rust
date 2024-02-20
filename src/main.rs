use error_chain::error_chain;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?; // the request is made

    //now we're simply printing some stuff
    println!("Status: {}", res.status());
    println!("Headers: \n {:#?}", res.headers());
    let body = res.text().await?;
    println!("Body: \n {}", body);
    Ok(())
}

// Asynch GET request in Rust
