use crate::{Context, Error};

/// Test Command: Download Game APK From ApkPure
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