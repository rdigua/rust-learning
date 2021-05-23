use std::collections::HashMap;
//use futures::future::ready;
//use std::future::Future;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}\n\n", resp);
//    Ok(())
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
    .text()
    .await?;
        println!("{:#?}\n\n", resp);
    //println!("body = {:?}", body);
/*
let content = reqwest::get("https://www.huj.cn/patterns.html")
    .await?
    .text()
    .await?;
*/

//println!("text: {}", content);
    let s=l_reqwest::gettext("https://www.huj.cn/patterns.html").await;
    println!("Text:{}",s);
    /*
    let c= reqwest::get("https://www.huj.cn/patterns.html").await?.text().await;


    let _ss=match c {
        Ok(s)=>println!("text: {}", s),
        Err(e)=>(),
    };
*/
    let _s=l_reqwest::gettext_async("https://www.jaytogo.com").await;
    Ok(())
}