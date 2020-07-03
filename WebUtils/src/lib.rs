#![allow(non_snake_case)]
use hyper::Client;
use hyper::{Body,Request};
use hyper_tls::HttpsConnector;
use bytes::Buf;
use serde_json::{Value};
use anyhow::Result;
use regex::Regex;

// TODO: rename to fetch_json
pub async fn get_json_value_from_url(fullUri: String) -> Result<Value> {

let client = get_https_client();
    // Parse an `http::Uri`...
let uri = fullUri.parse().unwrap();

println!("Doing request to: {:#?}",uri );

// Await the response...
let resp = client.get(uri).await.unwrap();

// asynchronously aggregate the chunks of the body
let mut body = hyper::body::aggregate(resp).await?;

//println!("has remaining {:#?}",  body.has_remaining());

let bodyString = String::from_utf8(body.to_bytes().to_vec())?;

//println!("Response body: {:#?}",  bodyString);

// try to parse as json with serde_json
let jsonValue : Value = serde_json::from_str(&bodyString)?;

Ok(jsonValue)

}

/**
 * 
 */
fn get_https_client() -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>
{
    
    let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

           client
}

/**
 * 
 */
pub async fn transform_text_into_shakespeare_text(text: String) ->  Result<String> {

    let client = get_https_client();

    //panic!("{:#?}", text);

    let params = r#"{"text": ":placeholder"}"#;

    let params = params.replace(":placeholder", &text);


    /*let re = Regex::new(r"\u{0000}-\u{001F}").unwrap();
    let mut params = re.replace_all(&text, "");*/

    //params.trim();

    //let mut params = params.replace_range("\u{0000}-\u{001F}", "");



    //let json: Value = serde_json::from_str(params.trim()).unwrap();

    //panic!("{:#?}", json);
 
      let req = Request::builder()
         .method("POST")
        .uri("https://api.funtranslations.com/translate/shakespeare.json")
        .body(Body::from(params))
         .expect("request builder");
   
     let res = client.request(req).await.unwrap();

     let JsonBodyString = String::from_utf8(hyper::body::to_bytes(res.into_body()).await?.to_vec())?;

     panic!("{:#?}", JsonBodyString);

     let jsonValue : Value = serde_json::from_str(&JsonBodyString)?;

     let jsonObj = jsonValue.as_object().expect("object should not be empty");

     panic!("params: {:#?}", jsonObj);

     let content = jsonObj["contents"]["translated"].as_str().expect("translation should be there");

     Ok(content.to_string())

}