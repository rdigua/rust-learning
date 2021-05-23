
use hyper_tls::HttpsConnector;
use hyper::{Client,Uri};
use std::fmt::Debug;    

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://hyper.rs".parse()?).await?;
    println!("Request is ok? {}",res.status());

    let client = Client::new();

    let ip_fut = async {
        let resp = client.get(Uri::from_static("https://www.huj.cn/")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

//    let headers_fut = async {
//        let resp = client.get(Uri::from_static("https://www.jaytogo.com/")).await?;
//        hyper::body::to_bytes(resp.into_body()).await
//    };
    
    // Wait on both them at the same time:
    //let (ip, headers) = futures::try_join!(ip_fut, headers_fut)?;
    futures::try_join!(ip_fut)?;
//Error: hyper::Error(Connect, "invalid URL, scheme is not http")
//error: process didn't exit successfully: `target\debug\l_re.exe` (exit code: 1)
    println!("{}",ip_fut);
//    assert_eq!(res.status(), 200);
    Ok(())


}
