#![allow(non_snake_case)]
use hyper::Client;
use hyper::{Body,Request};
use hyper_tls::HttpsConnector;
use bytes::Buf;
use serde_json::{Value};
use anyhow::Result;

/**
 * Fetch a json given a URI 
 * and transform it in a Json Value
 */
pub async fn fetch_json(fullUri: String) -> Result<Value> {

    //================
    // Get the Https client
    //================
    let client = get_https_client();

    //================
    // Build the uri
    //================
    let uri = fullUri.parse().unwrap();

    //================
    // Execute the api call 
    //================
    println!("Doing request to: {:#?}",uri );
    let resp = client.get(uri).await.unwrap();

    //================
    // Contruct the body string
    //================
    // asynchronously aggregate 
    // the chunks of the body
    let mut body = hyper::body::aggregate(resp).await?;
    let bodyString = String::from_utf8(body.to_bytes().to_vec())?;

    //================
    // Convert to Json Value
    //================
    let jsonValue : Value = serde_json::from_str(&bodyString)?;

    //================
    // Return
    //================
    Ok(jsonValue)

}

/**
 * Build an Https Client
 */
fn get_https_client() -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>
{   
    let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

           client
}

/**
 * Transform the text into a 
 * shakesperean text
 * 
 * NOTE: it could be a Job both on Pokemon module or in a shared one
 */
pub async fn transform_text_into_shakespeare_text(text: String) ->  Result<String> {

    //================
    // Get the Https client
    //================
    let client = get_https_client();
    
    //================
    // Construct Json String to be used in Body
    //================
    let params = r#"{"text": ":placeholder"}"#;
    let params = params.replace(":placeholder", &text);
 
    //================
    // Make the POST call
    //================
      let req = Request::builder()
         .method("POST")
        .uri("https://api.funtranslations.com/translate/shakespeare.json")
        .body(Body::from(params))
         .expect("request builder");
   
     let res = client.request(req).await.unwrap();

     //================
    // create the result Json value
    //================
     let JsonBodyString = String::from_utf8(hyper::body::to_bytes(res.into_body()).await?.to_vec())?;
    
     let jsonValue : Value = serde_json::from_str(&JsonBodyString)?;

     //================
    // Get the translated text
    //================
     let jsonObj = jsonValue.as_object().expect("object should not be empty");

     let content = jsonObj["contents"]["translated"].as_str().expect("translation should be there");

    //================
    // Result
    //================
     Ok(content.to_string())

}