use std::env;

use serenity::{
    model::{channel::Message},
    client::{Context},
    framework::standard::{
        macros::command,
        CommandResult
    }
};
use rss::Channel;

#[command]
async fn torrent(ctx: &Context, msg: &Message) -> CommandResult {
    let query = env::var("JACKETT_RSS_FEED").expect("Expected an RSS Feed Link");
    let content = reqwest::get(&query)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    println!("{:#?}", channel);

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("The Murgi screeches.");
            m.embed(|e| {
                e.title("The Murgi Lords grace you.");
                e.description("Here's a list of torrents that has been bestowed upon thee.");
                e.fields(vec![
                         ("Torrent 1", "magnet:?xt=urn:btih:2549775935A7CC05E771D1871E30BE1F48B440D2&dn=Shadow+of+the+Tomb+Raider%3A+Definitive+Edition+-+Team+Rjaa&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fexplodie.org%3A6969%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.tiny-vps.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce", true),
                         ("Torrent 1", "magnet:?xt=urn:btih:2549775935A7CC05E771D1871E30BE1F48B440D2&dn=Shadow+of+the+Tomb+Raider%3A+Definitive+Edition+-+Team+Rjaa&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fexplodie.org%3A6969%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.tiny-vps.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce", true),
                         ("Torrent 1", "magnet:?xt=urn:btih:2549775935A7CC05E771D1871E30BE1F48B440D2&dn=Shadow+of+the+Tomb+Raider%3A+Definitive+Edition+-+Team+Rjaa&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fexplodie.org%3A6969%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.tiny-vps.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce", true),
                         ("Torrent 1", "magnet:?xt=urn:btih:2549775935A7CC05E771D1871E30BE1F48B440D2&dn=Shadow+of+the+Tomb+Raider%3A+Definitive+Edition+-+Team+Rjaa&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fexplodie.org%3A6969%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.tiny-vps.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce", true),
                         ("Torrent 1", "magnet:?xt=urn:btih:2549775935A7CC05E771D1871E30BE1F48B440D2&dn=Shadow+of+the+Tomb+Raider%3A+Definitive+Edition+-+Team+Rjaa&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fexplodie.org%3A6969%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.tiny-vps.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce", true),
                         ("Torrent 1", "magnet:?xt=urn:btih:2549775935A7CC05E771D1871E30BE1F48B440D2&dn=Shadow+of+the+Tomb+Raider%3A+Definitive+Edition+-+Team+Rjaa&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fexplodie.org%3A6969%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.tiny-vps.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce", true),
                ]);
                e.field("Those were some of the top results", "If you didn't get what you want, then, maybe it's not here yet.", false);
                e.footer(|f| {
                    f.text("Jackett, Torrents, and Murgis.");
                    f
                });

                e.timestamp(chrono::Utc::now());
                e
            });
            m
        })
    .await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
