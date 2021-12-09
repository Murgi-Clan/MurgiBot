use std::env;

use serenity::{
    model::{channel::Message},
    client::{Context},
    framework::standard::{
        macros::command,
        CommandResult
    }
};

use rss::{Channel, Item};

#[command]
async fn torrent(ctx: &Context, msg: &Message) -> CommandResult {
    /// This function retrieves the message sent by the user, cleans it
    /// according to the number of characters in the command, creates a query,
    /// and then proceeds to retrieve the torrent data from the RSS Feed, and displays
    /// it in the form of an embed message in Discord.
    // Retrieving and cleaning the message content
    let mut s = String::from(&msg.content);
    let search = s.split_off(10);

    // Creating the query to be sent to Jackett
    let query = env::var("JACKETT_RSS_FEED").expect("Expected an RSS Feed Link") + &search;
    let content = reqwest::get(&query)
        .await?
        .bytes()
        .await?;

    // Creates a vector of items, where each item has information of one torrent.
    let channel:Vec<Item> = Channel::read_from(&content[..])?.items().to_vec();

    // Takes the first 5 responses and pushes them into the Vector.
    let mut torrs = vec![];
    for trt in 0..5 {
        torrs.push((channel[trt].title().unwrap(), channel[trt].link().unwrap(), true));
    }

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("The Murgi screeches.");
            m.embed(|e| {
                e.title("The Murgi Lords grace you.");
                e.description("Here's a list of torrents that has been bestowed upon thee.");
                // Need to convert this link into a string.
                e.url("https://www.youtube.com/watch?v=EGohSsaCJOU");
                // Probably can't put multiple links to stuff without it looking trash.
                e.fields(torrs);
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

    // Error Handling for the developer
    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
