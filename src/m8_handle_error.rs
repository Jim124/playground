use std::io::{Error, ErrorKind};

async fn async_call(url: &str) -> Result<serde_json::Value, Error> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;
    // .expect("could not retrieve the response");
    let json_response = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to json "))?;
    Ok(json_response)
}

#[cfg(test)]

mod test {
    use super::*;
    #[tokio::test]
    async fn tests_async_fn() {
        let api_url = "https://cat-fact.herokuapp.com/facts/";
        let response = async_call(api_url).await;
        match response {
            Ok(value) => dbg!(value),
            Err(_) => panic!("There was an error"),
        };
        let closure = || println!("hi ");
        closure();
    }
}
