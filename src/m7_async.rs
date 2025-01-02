async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    Ok(response)
}
#[cfg(test)]
mod test {

    use super::*;
    #[tokio::test]
    async fn tests_calls_async_fn() {
        let api_url = "https://cat-fact.herokuapp.com/facts/";
        let res = my_async_call(api_url).await;
        match res {
            Ok(value) => dbg!("{}", value),
            Err(_) => panic!("failed to make request!"),
        };
    }
}
