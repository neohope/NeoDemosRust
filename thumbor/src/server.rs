use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    handler::get,
    http::{HeaderMap, HeaderValue, StatusCode},
    Router,
};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{
    add_extension::AddExtensionLayer, compression::CompressionLayer, trace::TraceLayer,
};
use tracing::{info, instrument};
use std::convert::TryInto;

mod engine;
mod pb;

use engine::Photon;
use pb::*;

use crate::engine::Engine;

// 参数使用 serde 做 Deserialize，axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

// LRU缓存
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 初始化缓存
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));

    // 构建路由
    // 添加了三个处理层，获取图片，，
    let app = Router::new()
        // `GET /` 会执行
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(cache))
                .layer(CompressionLayer::new())
                .into_inner(),
        );

    // 运行 web 服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// get方法访问web服务器时，触发这个方法
async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    
    // pb转为ImageSpec
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // url字符串解除转码
    let url: &str = &percent_decode_str(&url).decode_utf8_lossy();

    // 从远程获取图片
    let data = retrieve_image(url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 使用 image engine 处理
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    // 转换输出为JPEG
    // TODO: 这里目前类型写死了，应该使用 content negotiation
    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("Finished processing: image size {}", image.len());

    // 输出图片
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, image))
}

// 从远程获取图片，同时做了一个缓存功能
#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    // 通过url计算key
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    // 缓存加锁
    // 如果缓存存在，直接返回
    // 如果缓存不存在，远程读取图片
    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)
}

// 调试辅助函数
fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}
