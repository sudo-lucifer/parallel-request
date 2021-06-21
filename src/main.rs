use futures::{stream , StreamExt};
use reqwest::Client;
use rayon::prelude::*;
use std::str;
use tokio;

const PARALLEL_REQUESTS_NUMBER: usize = 10;

// parallel asychronous HTTP GET Method
#[tokio::main]
async fn main(){
    // https://cs.muic.mahidol.ac.th/courses/ooc/
    let mainUrl = vec!["https://cs.muic.mahidol.ac.th/courses/ooc/"; PARALLEL_REQUESTS_NUMBER];

    // Make a new Client
    let client: Client = Client::new();

    // let body = reqwest::get("https://www.rust-lang.org")
    //     .await?
    //     .text()
    //     .await?;

    // println!("body = {:?}", body);

    // parallel sending request depending of PARALLEL_QUESTS_NUMBER
    let bodies = stream::iter(mainUrl).map(
        |url| {
            let client = client.clone();
            tokio::spawn(async move {
                let response  = client.get(url).header("Connection", "close").send().await?;
                println!("Status: {}",response.status());
                // println!("Headers:\n{:#?}", response.headers());
                response.bytes().await

            })
        }
    ).buffer_unordered(PARALLEL_REQUESTS_NUMBER);

    bodies.for_each(
        |result| async {
        match result {
            Ok(Ok(info)) => {
                // pattern matching
                // let convert = match str::from_utf8(&*info){
                //     Ok(v) => v,
                //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                // };
                // println!("result: {}", convert)
                println!("Result: {}", info.len())
            },
            Ok(Err(info)) => eprint!("Error: {}", info),
            // fail to join
            Err(e) => eprintln!("Join Error: {}", e),
        }
    }).await;

}
// fn main() {
//     println!("Hello, world!");
// }
