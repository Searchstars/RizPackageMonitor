// 禁用无意义警告
#! [allow(non_snake_case)]
#! [allow(non_camel_case_types)]
#! [allow(dead_code)]
#! [allow(unused_parens)]
#! [allow(non_upper_case_globals)]

mod structs;
mod urls;
mod tools;
mod commandtools;
mod commands;
mod loopthread;

use std::{fs, time::Duration};
use poise::serenity_prelude::{self as serenity, Client, Http};
use structs::Config;

static mut DClient: *mut Client = std::ptr::null_mut();
static mut DClientHttp: *mut Http = std::ptr::null_mut();

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>; 

fn get_config() -> Config{
    let config_string = fs::read_to_string("./config.json").expect("缺失配置文件! 请按照README中的格式在正确的路径下创建正确的配置文件后再试");
    let config: Config = serde_json::from_str(&config_string).expect("配置文件格式错误! 你真的有看README吗...");
    config
}

async fn guild_check(ctx: &Context<'_>) -> bool{
    if get_config().enable_dm_command == false {
        let guild_id_match = ctx.guild_id();
        match guild_id_match {
            Some(_) => {
                return true;
            },
            None => {
                ctx.defer_ephemeral().await.unwrap();
                ctx.reply("Sorry, this bot currently does not accept commands from private chats").await.unwrap();
                return false;
            },
        }
    }
    return true;
}

pub async fn process_update() -> String{
    if(!std::path::Path::new("./doing_update_process").exists()){
        fs::write("./doing_update_process", "true").unwrap();
        let mut reply_msg = "".to_string();
        reply_msg += &format!("版本更新 **v{}** -> **v{}** 自动解析 by RizPackageMonitor bot [转载请标明出处]\n\n", fs::read_to_string("./latest_check_ver").unwrap(), fs::read_to_string("./checked_new_ver").unwrap());
        reply_msg += "Catalog 热更资源列表对比分析结果如下：\n\n";
        if(std::path::Path::new("./latest_res").exists()){
            if(std::path::Path::new("./res_tmp").exists()){
                fs::remove_dir_all("./res_tmp").unwrap();
            }
            commandtools::download_game_resources().await;
            let cri_files_compare_ret = commandtools::compare_cri_files("./res_tmp","./latest_res");
            reply_msg += "      新增CRIWARE资源文件列表（基本就是曲子）：\n";
            for ret in cri_files_compare_ret{
                reply_msg += &format!("            {}\n", ret);
            }
        }
        else{
            reply_msg += "      此bot的运营人员似乎并没有留上一个版本的热更资源文件，或是错误的删除了上一个版本的热更资源文件，因此无法进行对比分析\n";
        }
        if(std::path::Path::new("./latest_assets").exists()){
            if(std::path::Path::new("./assets_tmp").exists()){
                fs::remove_dir_all("./assets_tmp").unwrap();
            }
            commandtools::extract_game_resources().await;
            let asset_files_compare_ret = commandtools::compare_asset_files("./assets_tmp","./latest_assets");
            reply_msg += &format!("\n针对资源文件的拆包对比分析结果如下：\n\n      新增图片资源文件 {} 张，其中：\n            曲绘 {} 张\n\n      新增铺面文件 {} 个，其中：\n            IN难度铺面新增 {} 张\n            HD难度铺面新增 {} 张\n            EZ难度铺面新增 {} 张\n\n注：命名不规范的铺面无法被识别\n\n", asset_files_compare_ret.extra_images_num, asset_files_compare_ret.extra_illust_num, asset_files_compare_ret.extra_chart_num, asset_files_compare_ret.extra_in_chart_num, asset_files_compare_ret.extra_hd_chart_num, asset_files_compare_ret.extra_ez_chart_num);
        }
        else{
            reply_msg += "\n针对资源文件的拆包对比分析结果如下：\n\n      此bot的运营人员似乎并没有留上一个版本的解包后资源文件，或是错误的删除了上一个版本的解包后资源文件，因此无法进行对比分析\n";
        }

        if(tools::folder_size("./assets_tmp") < 300.0 || tools::folder_size("./res_tmp") < 300.0){
            reply_msg = format!("内部错误，下载到的热更资源文件或解包得到的资源文件占用的存储空间大小小于预期（300.0M），故认定本次分析结果无效，建议retry\ntools::folder_size(\"./assets_tmp\") = {}\ntools::folder_size(\"./res_tmp\") = {}", tools::folder_size("./assets_tmp"), tools::folder_size("./res_tmp"));
            fs::write("./can_retry", fs::read_to_string("./checked_new_ver").unwrap()).unwrap();
        }

        fs::write("./latest_check_ver", fs::read_to_string("./checked_new_ver").unwrap()).unwrap();
        fs::remove_file("./checked_new_ver").unwrap();
        fs::remove_file("./doing_update_process").unwrap();
        
        if(!reply_msg.contains("内部错误，下载到的热更资源文件或解包得到的资源文件占用的存储空间大小小于预期")){
            if(std::path::Path::new("./latest_assets").exists()){
                fs::remove_dir_all("./latest_assets").unwrap();
            }
            if(std::path::Path::new("./latest_res").exists()){
                fs::remove_dir_all("./latest_res").unwrap();
            }
            fs::rename("./assets_tmp", "./latest_assets").unwrap();
            fs::rename("./res_tmp", "latest_res").unwrap();
        }

        return reply_msg;
    }
    else{
        return "已有正在进行的更新分析任务".to_string();
    }
}

/// Fuck this bot
#[poise::command(slash_command)]
async fn fucking(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.reply("dont fucking me pls".to_string()).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    //crate::commandtools::check_updates().await;

    let config = get_config();
    let token = config.token;
    let intents = serenity::GatewayIntents::non_privileged();

    let commands = vec![
        fucking(),
        commands::check_updates(),
        commands::testcmd_download(),
        commands::testcmd_downloadresources(),
        commands::testcmd_extractresources(),
        commands::testcmd_compare_cri_files(),
        commands::testcmd_compare_asset_files()
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                if config.enable_dm_command{
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }
                else{
                    let empty_commands: &[poise::Command<u64,u64>] = &[];
                    poise::builtins::register_globally(ctx, empty_commands).await?;
                }
                for guild_id in config.guild_id_list{
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(guild_id)).await?;
                    println!("已为Guild Id为 {} 的服务器注册命令", guild_id);
                }  
                println!("命令注册完成~");
                Ok(Data {})
            })
        })
        .build();
    
    unsafe {
        DClient = Box::into_raw(Box::new(serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap()));
        tokio::spawn((*DClient).start());
        DClientHttp = std::sync::Arc::into_raw((*DClient).http.clone()).cast_mut() as *mut poise::serenity_prelude::Http;
    }

    tokio::time::sleep(Duration::from_secs(5)).await;

    loop{
        loopthread::checkloop().await;
    }
}