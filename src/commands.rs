use crate::{Context, Error};

/// Test Command: Download Game APK
#[poise::command(slash_command)]
pub async fn testcmd_download(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.reply("Download request sended".to_string()).await?;
    tokio::spawn(crate::commandtools::download_game());
    Ok(())
}

/// Test Command: Download All Game Resources
#[poise::command(slash_command)]
pub async fn testcmd_downloadresources(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.reply("Download request sended".to_string()).await?;
    tokio::spawn(crate::commandtools::download_game_resources());
    Ok(())
}

/// Test Command: Extract All Game Resources
#[poise::command(slash_command)]
pub async fn testcmd_extractresources(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.reply("Extract request sended".to_string()).await?;
    tokio::spawn(crate::commandtools::extract_game_resources());
    Ok(())
}

/// Test Command: Compare CRIWARE Files
#[poise::command(slash_command)]
pub async fn testcmd_compare_cri_files(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let compare_ret = crate::commandtools::compare_cri_files("./res_tmp","./latest_res");
    let mut reply_str = "".to_string();
    for ret in compare_ret{
        reply_str += &format!("      {}\n", ret);
    }
    ctx.reply(format!("Compare Result: \n{}",reply_str)).await?;
    Ok(())
}

/// Test Command: Compare Asset Files
#[poise::command(slash_command)]
pub async fn testcmd_compare_asset_files(ctx: Context<'_>) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;
    let compare_ret = crate::commandtools::compare_asset_files("./assets_tmp","./latest_assets");
    let mut reply_str = "".to_string();
    reply_str += &format!("针对资源文件的拆包对比分析结果如下：\n\n      新增图片资源文件 {} 张，其中：\n            曲绘 {} 张\n\n      新增铺面文件 {} 个，其中：\n            IN难度铺面新增 {} 张\n            HD难度铺面新增 {} 张\n            EZ难度铺面新增 {} 张\n\n注：命名不规范的铺面无法被识别\n\n", compare_ret.extra_images_num, compare_ret.extra_illust_num, compare_ret.extra_chart_num, compare_ret.extra_in_chart_num, compare_ret.extra_hd_chart_num, compare_ret.extra_ez_chart_num);
    ctx.reply(reply_str).await?;
    Ok(())
}

/// 立即检查游戏更新
#[poise::command(slash_command)]
pub async fn check_updates(ctx: Context<'_>) -> Result<(), Error> {
    if(crate::guild_check(&ctx)).await{
        ctx.defer_ephemeral().await?;
        let update_ret = crate::commandtools::check_updates().await;
        if(update_ret){
            if(!std::path::Path::new("./doing_update_process").exists()){
                ctx.reply("我去，还真有更新！\n并且...还没有正在进行的分析任务！\n感谢你的CheckUpdates请求，分析请求已提交！").await?;
                tokio::spawn(crate::process_update());
            }
            else{
                ctx.reply("我去，还真有更新！\n不过吧...似乎已经有正在分析的任务了\n你来晚啦~不过还是感谢奥！").await?;
            }
        }
        else{
            ctx.reply("看起来并没有更新呢...如果是官方社交媒体发了更新公告的话，要在这里检查到更新也得等到更新维护结束后了，所以别刷这个Bot，去刷官号行不？\n\n当然要是更新维护结束后还没法检查到更新的话，那问题可就大了...去找找此Bot的运营人员吧！").await?;
        }
    }
    Ok(())
}