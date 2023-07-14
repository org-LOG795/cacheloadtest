use std::io;

use argparse::{ArgumentParser, StoreTrue, Store};


use tokio;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, AsyncSeekExt};
use futures::future::join_all;
use std::io::SeekFrom;
use reqwest::{Client};



#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut destination = "".to_string();
    let mut payload_size = 512;
    let mut range = 26;

    // args 
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Load tester");
        ap.refer(&mut payload_size)
            .add_option(&["-s", "-size"], Store, "Specify each payload size that will be sent to the endpoint");
        ap.refer(&mut range)
            .add_option(&["-r","-range"], Store, "Specify the range of diferent data to be sent (Usually between 0 and 26 [A-Z]");
        ap.refer(&mut destination)
            .add_argument("destination", Store, "url destination to test");

        ap.parse_args_or_exit();

    }

    let mut handles = vec![];

    for i in 0..range {
        let dest = destination.clone();
        handles.push( tokio::spawn(async move {

            println!("Task {} started", i);
            // let mut file = OpenOptions::new()
            //     .append(true)
            //     .create(true)
            //     .open("tmp/crawler5.txt")
            //     .await.expect("Open file fails"); 

            // let start_pos = file.seek(SeekFrom::Current(0)).await.unwrap();

            let payload = char::from_u32(i as u32 +'A' as u32).unwrap().to_string().repeat(payload_size)    ;

            let client = reqwest::Client::new();
            // let form = reqwest::blocking::multipart::Form::new()
            // .text("key3", "value3")
            // .file("file", "/path/to/field")?;
            let response = client.post(dest).body(payload).send().await.unwrap();
            
            // let payload = i.to_string()+"\n";
            // let plen = payload.as_bytes().len();
            
            // let wlen = file.write_all(payload.as_bytes()).await.expect("Error writing");

            // let end_pos = file.seek(SeekFrom::Current(0)).await.unwrap();

            // let start_pos = end_pos-plen as u64;

            println!("{}: Status:{}", i, response.status().as_str() )

        }));
    }

    futures::future::join_all(handles).await;

    Ok(())
}