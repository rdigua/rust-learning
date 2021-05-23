pub async fn gettext<T: AsRef<str>>(link: T) -> String {
    let mut r: String = "".to_string();
    let re = reqwest::get(link.as_ref()).await;
    match re {
        Ok(re) => {
            match re.text().await {
                Ok(s) => r = s,
                _ => return r,
            }
        }
        _ => return r,
    }
    r
    /*
    fn async fn gettext<T: AsRef<str>>(link:T)-> Result<String, Box<dyn std::error::Error>>{
    let content = reqwest::get("https://www.huj.cn/patterns.html")
    .await?
    .text()
    .await?;
    content
    }
     */
}

pub async fn gettext_async<T: AsRef<str>>(link: T) -> Result<String, Box<dyn std::error::Error>> {
    let content = reqwest::get(link.as_ref())
        .await?
        .text()
        .await?;
    Ok(content)
}