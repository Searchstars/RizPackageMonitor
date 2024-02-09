use std::time::Duration;

pub fn string_vec_contains_string(vec: Vec<&String>, s: &String) -> bool {
    for i in vec.clone() {
        if i == s {
            return true;
        }
        else{
            //println!("{} != {}",i,s);
        }
    }
    println!("{}", s);
    let mut tmp_vecdir2 = "".to_string();
    for s in vec{
        tmp_vecdir2 += &("\n".to_string() + s.as_str());
    }
    std::fs::write("./dbg_tmp", tmp_vecdir2).unwrap();
    false
}

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

pub async fn httpreq_post(url: String, body: String, content_type: String) -> String{
    let client = reqwest::Client::new();
    let body = client.post(url).header("Content-Type", content_type).body(body).send().await.unwrap().text().await.unwrap();
    return body;
}

pub async fn assetripper_httpreq_post_path_form(url: String, path: String, content_type: String) -> String {
    // 创建一个 reqwest 的客户端
    let client = reqwest::Client::new();
    // 创建一个 Part，设置文本内容和 MIME 类型
    let path_part = reqwest::multipart::Part::text(path)
        .mime_str(&content_type)
        .unwrap();
    // 创建一个 Form，添加一个名为 "Path" 的文本字段
    let form = reqwest::multipart::Form::new()
        .part("Path", path_part);
    // 使用客户端发送 POST 请求，设置请求体为 form
    let body = client
        .post(url)
        .multipart(form)
        .timeout(Duration::from_secs(114514))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // 返回响应体
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

/// 传入文件夹路径，获取文件夹大小，一般用于判定一个文件夹是否有效，资源下载是否正确完成等
pub fn folder_size(path: &str) -> f32 {
    // 初始化一个变量，用于累计文件夹内所有文件的大小，以字节为单位
    let mut total_size = 0;

    // 使用fs::read_dir方法，遍历文件夹内的所有条目
    for entry in std::fs::read_dir(path).unwrap() {
        // 获取每个条目的元数据，包括文件类型和大小
        let metadata = entry.as_ref().unwrap().metadata().unwrap();

        // 如果条目是一个文件，就把它的大小加到total_size上
        if metadata.is_file() {
            total_size += metadata.len();
        }

        // 如果条目是一个子文件夹，就递归调用folder_size函数，把它的大小加到total_size上
        if metadata.is_dir() {
            total_size += folder_size(entry.as_ref().unwrap().path().to_str().unwrap()) as u64;
        }
    }

    // 把total_size转换为f32类型，并除以1024 * 1024，得到以M为单位的结果
    total_size as f32 / (1024.0 * 1024.0)
}