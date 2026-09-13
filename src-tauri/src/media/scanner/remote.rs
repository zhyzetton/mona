use std::time::Duration;

use crate::database::repository::Repository;
use crate::errors::AppError;
use crate::media::model::Media;
use crate::media::scanner::Scanner;
use async_trait::async_trait;
use reqwest::Client;

pub struct RemoteScanner {
    client: Client,
    base_url: String,
    root: String,
}

impl RemoteScanner {
    pub fn new(
        url: String,
        username: String,
        password: String,
        root: String,
    ) -> Result<RemoteScanner, AppError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::WebDav(format!("构建 HTTP 客户端失败: {e}")))?;
        Ok(Self {
            client,
            base_url: url.trim_end_matches('/').to_string(),
            root,
        })
    }
}

#[async_trait]
impl Scanner for RemoteScanner {
    async fn scan(&self, repo: &Repository) -> Result<Vec<Media>, AppError> {
        Ok(vec![])
    }

    fn name(&self) -> &'static str {
        "Remote"
    }
}

mod tests {
    use reqwest::{Client, Method};
    use std::time::Duration;

    #[tokio::test]
    async fn test_webdav_list() {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        let body = r#"<?xml version="1.0" encoding="UTF-8"?>
            <d:propfind xmlns:d="DAV:">
                <d:prop>
                    <d:getcontentlength/>
                    <d:getlastmodified/>
                    <d:resourcetype/>
                </d:prop>
            </d:propfind>"#;
        let resp = client
            .request(
                Method::from_bytes(b"PROPFIND").unwrap(),
                "http://127.0.0.1:5244/dav/QuarkDrive/电影",
            )
            .basic_auth("admin", Some("admin"))
            .header("Depth", "infinity")
            .header("Content-Type", "application/xml; charset=utf-8")
            .body(body)
            .send()
            .await
            .expect("请求失败");
        println!("HTTP 状态: {}", resp.status());
        let xml = resp.text().await.expect("读取响应文本失败");
        println!("---- 原始 XML ----");
        println!("{}", xml);
        println!("---- XML 结束 ----");
    }
}
