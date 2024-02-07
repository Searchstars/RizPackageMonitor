use std::fs;
use std::path::Path;

/// 检查游戏更新，通过对比存储在latest_check_ver中的版本号来判断，如果该文件不存在，则返回false（没有更新）并将当前最新版本号创建并写入该文件
pub async fn check_updates() -> bool{
    let game_config = crate::tools::get_game_config().await;
    if(Path::new("latest_check_ver").exists()){
        for config in game_config.configs{
            if(version_compare::Version::from(&config.version) > version_compare::Version::from(&fs::read_to_string("latest_check_ver").unwrap())){
                return true;
            }
        }
        return false;
    }
    else{
        fs::write("latest_check_ver", game_config.configs[0].version.clone()).unwrap();
        return false;
    }
}

/// 下载游戏apk包体，先从apkpure下载，如果失败切apkcombo，还是不行就返回false
pub async fn download_game() -> bool{
    // 尝试从apkpure下载游戏apk
    crate::tools::download_file(&crate::urls::game_apkpure_download_url(), "./tmp_apk.apk".to_string()).await;
    if((!std::path::Path::new("./tmp_apk.apk").exists()) || (crate::tools::get_file_size_in_mb("./tmp_apk.apk") < 50.0)){
        println!("文件疑似下载失败, 尝试从ApkCombo下载");
        let apkcombo_html = crate::tools::httpreq_get(crate::urls::game_apkcombo_download_page()).await;
        let apkcombo_down_url_splits: Vec<&str> = apkcombo_html.split("<a href=\"https://download.apkcombo.com/com.").collect();
        let apkcombo_down_url_backstring = apkcombo_down_url_splits[1].to_string();
        let apkcombo_down_url_backsplits: Vec<&str> = apkcombo_down_url_backstring.split(" class=\"variant\" rel=\"nofollow noreferrer\">").collect();
        let apkcombo_down_url = apkcombo_down_url_backsplits[0].to_string() + "&";
        let apkcombo_checkin_result = crate::tools::httpreq_get("https://apkcombo.com/checkin".to_string()).await;
        print!("ApkCombo Checkin 结果：{}  ApkCombo Download URL: {}", apkcombo_checkin_result, apkcombo_down_url);
        crate::tools::download_file(&("https://download.apkcombo.com/com.".to_string() + &apkcombo_down_url + &apkcombo_checkin_result), "./tmp_apk.apk".to_string()).await;
        if((!std::path::Path::new("./tmp_apk.apk").exists()) || (crate::tools::get_file_size_in_mb("./tmp_apk.apk") < 50.0)){
            return false;
        }
    }
    return true;
}

/// 下载所有游戏资源文件到./res_tmp目录下，如果没有该文件夹会自动创建，后续资源的对比和移动需要自行操作
pub async fn download_game_resources(){
    let game_config = crate::tools::get_game_config().await;
    for config in game_config.configs{
        if(config.version == fs::read_to_string("./latest_check_ver").unwrap()){
            let resource_url_str = config.resourceUrl.as_str();
            let catalog_string = crate::tools::httpreq_get(config.resourceUrl.to_string() + &"/Android/catalog_catalog.json").await;
            let catalog: crate::structs::Catalog = serde_json::from_str(&catalog_string).unwrap();
            if(!Path::new("./res_tmp").exists()){
                fs::create_dir("./res_tmp").unwrap();
            }
            for InternalId in catalog.m_InternalIds{
                if(InternalId.contains("http://") && InternalId.contains("/default/")){
                    let mut download_url = InternalId.clone().replace("http://rizastcdn.pigeongames.cn/default", resource_url_str);
                    if(InternalId.contains("/cridata_assets_criaddressables/")){
                        download_url = download_url.replace(".bundle", "");
                    }
                    let download_url_splits: Vec<&str> = download_url.split("/").collect();
                    let file_name = download_url_splits[download_url_splits.len() - 1];
                    crate::tools::download_file(&download_url, "./res_tmp/".to_string() + &file_name).await;
                }
            }
            println!("下载完成");
        }
    }
}

/// 将所有下载到的游戏资源拆包并放到临时文件夹中，需要res_tmp文件夹存在，否则直接报错
pub async fn extract_game_resources(){
    if(!Path::new("./assets_tmp").exists()){
        fs::create_dir("./assets_tmp").unwrap();
    }
    println!("当前运行路径: {}", std::env::current_dir().unwrap().to_string_lossy());
    println!("开始加载bundle文件夹");
    crate::tools::assetripper_httpreq_post_path_form(crate::get_config().assetripper_server_url + "/LoadFolder", (std::env::current_dir().unwrap().to_string_lossy() + "/res_tmp/").to_string(), "multipart/form-data; boundary=&".to_string()).await;
    println!("bundle文件加载完毕, 开始导出");
    crate::tools::assetripper_httpreq_post_path_form(crate::get_config().assetripper_server_url + "/Export", (std::env::current_dir().unwrap().to_string_lossy() + "/assets_tmp").to_string(), "multipart/form-data; boundary=&".to_string()).await;
    println!("导出操作现已完成");
}

/// 对比两个res文件夹中的音乐资源，并返回dir1比dir2多出来的新音乐的名称列表
fn compare_cri_files(dir1: &str, dir2: &str) -> Vec<String> {
    // 定义一个空的向量，用于存储结果
    let mut result = Vec::new();

    // 定义一个闭包，用于判断文件名是否包含两个关键字
    let is_target = |name: &str| name.contains(".") && name.contains("acb=");

    // 遍历第一个文件夹中的所有文件
    for entry in fs::read_dir(dir1).unwrap() {
        // 获取文件路径
        let path = entry.unwrap().path();
        // 如果是文件，并且是目标文件
        if path.is_file() && is_target(path.file_name().unwrap().to_str().unwrap()) {
            // 获取文件名
            let name = path.file_name().unwrap().to_str().unwrap().to_owned();
            // 如果第二个文件夹中不存在同名文件，或者同名文件不是目标文件
            if !Path::new(dir2).join(&name).exists()
                || !is_target(Path::new(dir2).join(&name).file_name().unwrap().to_str().unwrap())
            {
                // 将文件名加入结果向量
                result.push(name);
            }
        }
    }

    // 返回结果向量
    result
}