use std::fs;
use std::path::Path;

/// 检查游戏更新，通过对比存储在latest_check_ver中的版本号来判断，如果该文件不存在，则返回false（没有更新）并将当前最新版本号创建并写入该文件
pub async fn check_updates() -> bool{
    let game_config = crate::tools::get_game_config().await;
    if(Path::new("latest_check_ver").exists()){
        for config in game_config.configs{
            if(version_compare::Version::from(&config.version) > version_compare::Version::from(&fs::read_to_string("latest_check_ver").unwrap())){
                fs::write("checked_new_ver", &config.version).unwrap();
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

static mut CURRENT_DOWNLOAD_URL: &str = "";

/// 下载所有游戏资源文件到./res_tmp目录下，如果没有该文件夹会自动创建，后续资源的对比和移动需要自行操作
pub async fn download_game_resources(){
    let game_config = crate::tools::get_game_config().await;
    for config in game_config.configs{
        if(config.version == fs::read_to_string("./checked_new_ver").unwrap()){
            let resource_url_str = config.resourceUrl.as_str();
            let catalog_string = crate::tools::httpreq_get(config.resourceUrl.to_string() + &"/Android/catalog_catalog.json").await;
            let catalog: crate::structs::Catalog = serde_json::from_str(&catalog_string).unwrap();
            if(!Path::new("./res_tmp").exists()){
                fs::create_dir("./res_tmp").unwrap();
            }
            // 创建一个空的 FuturesUnordered 集合
            let mut download_tasks = futures::stream::FuturesUnordered::new();
            for InternalId in catalog.m_InternalIds{
                if(InternalId.contains("http://") && InternalId.contains("/default/")){
                    let mut download_url = InternalId.clone().replace("http://rizastcdn.pigeongames.cn/default", resource_url_str);
                    if(InternalId.contains("/cridata_assets_criaddressables/")){
                        download_url = download_url.replace(".bundle", "");
                    }
                    let download_url_vecu8 = (&download_url).clone().into_bytes();
                    let download_url_splits: Vec<&str> = download_url.split("/").collect();
                    let file_name = download_url_splits[download_url_splits.len() - 1];
                    // 将每个下载任务加入到集合中，而不是立即执行
                    download_tasks.push(async {
                        // 使用 reqwest::get 异步发送 GET 请求，等待响应
                        let response = reqwest::get(download_url).await.expect("请求失败");
                        let download_url_clone = crate::tools::vec_to_string(download_url_vecu8).unwrap();
                        // 我们史山代码是这样的 可是没办法 在另一个线程里确实就得重新实现一遍
                        let download_url_splits: Vec<&str> = download_url_clone.split("/").collect();
                        let file_name = download_url_splits[download_url_splits.len() - 1];
                        // 使用 bytes 方法获取响应的二进制数据
                        let data = response.bytes().await.expect("获取数据失败");
                        // 使用 std::fs::write 函数将数据写入到 save_path 指定的文件中
                        std::fs::write(format!("./res_tmp/{}", file_name), data).expect("写入文件失败");
                    });
                }
            }
            // 等待所有的下载任务完成
            while let Some(result) = futures::StreamExt::next(&mut download_tasks).await {
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
pub fn compare_cri_files(dir1: &str, dir2: &str) -> Vec<String> {
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

/// 对比dir1中比dir2中多了多少张图片，多少张铺面等
pub fn compare_asset_files(dir1: &str, dir2: &str) -> crate::structs::AssetFolderCompareResult {
    // 定义一个图片扩展名的向量
    let image_exts = vec!["png", "jpg", "jpeg", "gif", "bmp", "svg"];

    // 定义铺面和曲绘的关键词常量
    const CHART_IN_WORD: &str = "Chart_IN_";
    const CHART_HD_WORD: &str = "Chart_HD_";
    const CHART_EZ_WORD: &str = "Chart_EZ_";
    const CHART_WORD: &str = "Chart_";
    const ILLUST_WORD: &str = "illustration.";

    // 使用 HashSet 来存储 dir2 中的文件名
    let mut dir2_files = std::collections::HashSet::new();

    // 使用 walkdir 库，递归地遍历 dir2 中的文件
    for entry in walkdir::WalkDir::new(dir2) {
        // 如果 entry 是一个文件，获取它的名称并转换为 String 类型
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    //println!("{} 被插入到HashSet", &name);
                    // 如果它是个文件，就将名称插入到 HashSet 中
                    dir2_files.insert(name.to_string());
                }
            }
        }
    }

    let mut extra_images_num = 0;
    let mut extra_in_chart_num = 0;
    let mut extra_hd_chart_num = 0;
    let mut extra_ez_chart_num = 0;
    let mut extra_chart_num = 0;
    let mut extra_illust_num = 0;

    for dir2_file in &dir2_files{
        //println!("{}\n",dir2_file);
    }

    let dir2_files_vec = dir2_files.iter().collect::<Vec<&String>>();

    // 使用 walkdir 库，递归地遍历 dir1 中的文件
    for entry in walkdir::WalkDir::new(dir1) {
        // 如果 entry 是一个文件，获取它的名称并转换为 String 类型
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    let name = name.to_string();
                    if let Some(ext) = name.split('.').last() {
                        // 如果扩展名是图片扩展名，且名称不在 HashSet 中，将发现的图片数量自增
                        if image_exts.contains(&ext) && !dir2_files.contains(&name) {
                            extra_images_num += 1;
                        }
                    }
                    //println!("{}",name);
                    if name.contains(CHART_IN_WORD) && name.contains(".json"){
                        println!("文件{}含有CHART_IN_WORD和.json后缀", &name);
                        if (!crate::tools::string_vec_contains_string(dir2_files_vec.clone(), &name)) && (!name.contains(".meta")){
                            println!("Passed -> 文件{}不在dir2_files列表中且不含有.meta后缀", &name);
                            extra_in_chart_num += 1;
                        }
                    }
                    if name.contains(CHART_HD_WORD) && name.contains(".json") && (!crate::tools::string_vec_contains_string(dir2_files_vec.clone(), &name)) && (!name.contains(".meta")){
                        extra_hd_chart_num += 1;
                    }
                    if name.contains(CHART_EZ_WORD) && name.contains(".json") && (!dir2_files.contains(&name)) && (!name.contains(".meta")){
                        extra_ez_chart_num += 1;
                    }
                    if name.contains(CHART_WORD) && name.contains(".json") && (!dir2_files.contains(&name)) && (!name.contains(".meta")){
                        extra_chart_num += 1;
                    }
                    if name.contains(ILLUST_WORD) && name.contains(".png") && name.contains(".0.") && (!dir2_files.contains(&name)) && (!name.contains(".meta")){
                        extra_illust_num += 1;
                    }
                }
            }
        }
    }

    // 返回一个结构体，包含各种类型的文件数量的差异
    crate::structs::AssetFolderCompareResult {
        extra_images_num,
        extra_in_chart_num,
        extra_hd_chart_num,
        extra_ez_chart_num,
        extra_chart_num,
        extra_illust_num
    }
}