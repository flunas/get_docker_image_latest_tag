use reqwest;
use serde_json::Value;
use std::error::Error;

async fn fetch_latest_tags(image: &str, library: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut all_tags = Vec::new();
    let mut next_url = Some(format!(
        "https://hub.docker.com/v2/repositories/{}/{}/tags/?page=1&page_size=1",
        library.unwrap_or("library"),
        image
    ));

    let client = reqwest::Client::new();
    while let Some(url) = &next_url {
        let resp: Value = client.get(url).send().await?.json().await?;

        // 如果没找到
        if resp["message"] == "object not found" {
            return Err("请添加第二个参数".into());
        }

        // 提取当前页的标签名
        if let Some(results) = resp["results"].as_array() {
            for tag in results {
                if let Some(name) = tag["name"].as_str() {
                    all_tags.push(name.to_string());
                }
            }
        }

        // 检查是否有下一页
        // next_url = resp["next"].as_str().map(|s| s.to_string());
        next_url = None;
    }

    Ok(all_tags)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut image_name = "";
    let mut library_name:Option<&str> = None;
    if args.len() == 1 {
        eprintln!("参数错误");
        eprintln!("Usage: cargo run -- <image_name> [library_name=library]");
        return Ok(());
    } else if args.len() == 2 {
        image_name = &args[1];
    } else if args.len() == 3 {
        image_name = &args[1];
        library_name = Some(&args[2]);
    } else {
        eprintln!("参数错误");
        eprintln!("Usage: cargo run -- <image_name> [library_name=library]");
        return Ok(());
    }
    let tags = match fetch_latest_tags(image_name, library_name).await {
        Ok(tags) => tags,
        Err(e) => {
            if args.len() == 3 {
                println!("0.1.0");
                return Ok(());
            } else {
                eprintln!("Error: {}, 因为在默认的library中找不到{}镜像", e, args[1]);
                eprintln!("Usage: cargo run -- {} <如果是个人的镜像,该参数为用户名>", args[1]);
                return Ok(());
            }
        }
    };
    if tags.is_empty() {
        println!("0.1.0");
    } else if tags.len() == 1 {
        println!("{}", tags[0]);
    }
    Ok(())
}