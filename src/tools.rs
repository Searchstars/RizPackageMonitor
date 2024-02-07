/// 将Vec<u8>转换为String
pub fn vec_to_string(v: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    let s = String::from_utf8(v);
    s
}

pub async fn httpreq_get(url: String) -> String{
    let client = reqwest::Client::new();
    let body = client.get(url).send().await.unwrap().text().await.unwrap();
    return body;
}

pub async fn httpreq_post(url: String, body: String) -> String{
    let client = reqwest::Client::new();
    let body = client.post(url).body(body).send().await.unwrap().text().await.unwrap();
    return body;
}

pub async fn get_game_config() -> crate::structs::GameConfig{
    let ret = httpreq_get(crate::urls::game_config_url()).await;
    let game_config: crate::structs::GameConfig = serde_json::from_str(&ret).unwrap();
    return game_config;
}

/// 下载文件，成功或失败需要调用后自行判断
pub async fn download_file(url: &String, save_path: String) {
    println!("Tools::download_file > 收到请求，URL={}", url);
    // 使用 reqwest::get 异步发送 GET 请求，等待响应
    let response = reqwest::get(url).await.expect("请求失败");
    // 检查响应状态码是否为 200，表示成功
    if response.status() == 200 {
        // 使用 bytes 方法获取响应的二进制数据
        let data = response.bytes().await.expect("获取数据失败");
        // 使用 std::fs::write 函数将数据写入到 save_path 指定的文件中
        std::fs::write(&save_path, data).expect("写入文件失败");
        // 打印一条成功信息
        println!("文件 {} 已经成功下载到 {}", url, save_path);
    } else {
        // 如果状态码不为 200，打印一条错误信息
        println!("文件 {} 下载失败，状态码为 {}", url, response.status());
    }
}

/// 获取文件大小，单位为 MB
pub fn get_file_size_in_mb(filename: &str) -> f64 {
    // 使用 fs::metadata 函数获取文件的元数据，使用 unwrap 处理错误
    let metadata = std::fs::metadata(filename).unwrap();
    // 使用 len 方法获取文件的字节数，转换为 f64 类型
    let bytes = metadata.len() as f64;
    // 使用公式 MB = 字节 / 1000000 计算文件的 MB 大小
    let mb = bytes / 1000000.0;
    // 返回 mb 值
    mb
}