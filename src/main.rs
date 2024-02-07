// 禁用无意义警告
#! [allow(non_snake_case)]
#! [allow(non_camel_case_types)]
#! [allow(dead_code)]
#! [allow(unused_parens)]

mod structs;
mod urls;
mod tools;
mod commandtools;
mod commands;

use std::fs;
use poise::serenity_prelude::{self as serenity};
use structs::Config;

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
    crate::commandtools::check_updates().await;

    let config = get_config();
    let token = config.token;
    let intents = serenity::GatewayIntents::non_privileged();

    let commands = vec![
        fucking(),
        commands::testcmd_download(),
        commands::testcmd_downloadresources()
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

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap();
    client.start().await.unwrap();
}