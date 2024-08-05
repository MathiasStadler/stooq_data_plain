

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`


use  std::fs;




#[tokio::main]
async fn main()  {

    // output file
    let mut file = fs::OpenOptions::new()
                            .append(true)
                            .open("stock_data.txt");

    // Some simple CLI args requirements...
    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        println!("No CLI URL provided, using default.");
        "https://hyper.rs".into()
    };

    eprintln!("Fetching {url:?}...");

    // reqwest::get() is a convenience function.
    //
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let res = reqwest::get(url).await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    file.write_all_at(b"{}",body);
    println!("{body}");

    
}

