use serenity::{
    model::{channel::Message},
    client::{Context},
    framework::standard::{
        macros::command,
        CommandResult
    }
};

#[command]
async fn howl(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "MURGI CLAN AWOOOOOOOOOOOO").await?;

    Ok(())
}
