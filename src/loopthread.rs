use std::time::Duration;

pub async fn testloop(){
    tokio::time::sleep(Duration::from_secs(3)).await;
    let client = unsafe{ &(*crate::DClient)};
    let channel = client.cache.channel(1201521864669933618).unwrap();
    channel.say(client.http.clone(), "test message").await.unwrap();
}

pub async fn checkloop(){
    if(crate::commandtools::check_updates().await){
        let client = unsafe{ &(*crate::DClient)};
        let channel = client.cache.channel(crate::get_config().auto_check_send_channelid).unwrap();
        channel.say(client.http.clone(), "侦测到游戏版本更新！开始处理分析......").await.unwrap();
        let send_content = crate::process_update().await;
        channel.say(client.http.clone(), send_content).await.unwrap();
    }
    tokio::time::sleep(Duration::from_secs(30*60)).await;
}