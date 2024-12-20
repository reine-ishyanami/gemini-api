#![allow(deprecated)]

use std::env;

use anyhow::Result;
use gemini_api::model::Gemini;
use gemini_api::param::LanguageModel;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_chat_once() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    let req1: String = "My Name is Reine".into();
    let resp1 = client.chat_once(req1.clone()).await?;
    assert!(!resp1.is_empty());
    println!("{}: {}", req1, resp1);
    Ok(())
}

#[tokio::test]
async fn test_chat_conversation() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    let req1 = "My Name is Reine".to_owned();
    let resp1 = client.chat_conversation(req1.clone()).await?;
    assert!(!resp1.is_empty());
    let req2 = "Who am I".to_owned();
    let resp2 = client.chat_conversation(req2.clone()).await?;
    assert!(!resp2.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_chat_with_system_instruction() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    client.set_system_instruction("你是 Reine ".into());
    let resp = client.chat_once("你是谁？".into()).await?;
    assert!(!resp.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_chat_conversation_with_system_instruction() -> Result<()> {
    sleep(Duration::from_secs(60)).await;
    let key = env::var("GEMINI_KEY");
    assert!(key.is_ok());
    let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
    client.set_system_instruction("你是一只猫娘，你每次说话都会在句尾加上喵~ ".into());
    let resp1 = client.chat_conversation("My Name is Reine".into()).await?;
    assert!(!resp1.is_empty());
    let resp2 = client.chat_conversation("Who am I".into()).await?;
    assert!(!resp2.is_empty());
    Ok(())
}

#[tokio::test]
#[cfg(feature = "image_analysis")]
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
