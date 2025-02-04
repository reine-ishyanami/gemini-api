#![allow(deprecated, unused_imports)]

use std::env;

use gemini_api::model::Gemini;
use gemini_api::param::LanguageModel;
use tokio::time::{sleep, Duration};

#[tokio::test]
#[cfg(feature = "image_analysis")]
#[cfg(not(feature = "blocking"))]
async fn test_image_analysis() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    let image_path = r#"./file_type_rust.png"#;
    let resp = client
        .image_analysis(image_path.into(), "分析一下这张图片".into())
        .await?;
    assert!(!resp.is_empty());
    println!("{}", resp);
    Ok(())
}

#[tokio::test]
#[cfg(feature = "image_analysis")]
#[cfg(not(feature = "blocking"))]
async fn test_image_analysis_network() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    let image_path = "https://img.loliapi.cn/i/pp/img3.webp";
    let resp = client
        .image_analysis(image_path.into(), "分析一下这张图片".into())
        .await?;
    assert!(!resp.is_empty());
    println!("{}", resp);
    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "blocking"))]
async fn test_send_simple_message_once() {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    let req1 = "My Name is Reine".to_owned();
    let resp1 = client.send_simple_message(req1.clone()).await;
    assert!(resp1.is_ok());
    println!("{}: {}", req1, resp1.unwrap().0);
}

#[tokio::test]
#[cfg(not(feature = "blocking"))]
async fn test_send_simple_message_conversation() {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    client.start_chat(Vec::new());
    let req1 = "My Name is Reine".to_owned();
    let resp1 = client.send_simple_message(req1.clone()).await;
    assert!(resp1.is_ok());
    println!("{}: {}", req1, resp1.unwrap().0);
    let req2 = "Who am I".to_owned();
    let resp2 = client.send_simple_message(req2.clone()).await;
    assert!(resp2.is_ok());
    println!("{}: {}", req2, resp2.unwrap().0);
}

#[tokio::test]
#[cfg(feature = "image_analysis")]
#[cfg(not(feature = "blocking"))]
async fn test_send_image_message() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    let image_path = r#"./file_type_rust.png"#;
    let (resp, _) = client
        .send_image_message(image_path.into(), "分析一下这张图片".into())
        .await?;
    assert!(!resp.is_empty());
    println!("{}", resp);
    Ok(())
}

#[tokio::test]
#[cfg(feature = "image_analysis")]
#[cfg(not(feature = "blocking"))]
async fn test_send_image_message_network() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    let image_path = "https://img.loliapi.cn/i/pp/img3.webp";
    let (resp, _) = client
        .send_image_message(image_path.into(), "分析一下这张图片".into())
        .await?;
    assert!(!resp.is_empty());
    println!("{}", resp);
    Ok(())
}
