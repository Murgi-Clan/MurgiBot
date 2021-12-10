use serenity::{
    model::{channel::Message},
    client::{Context},
    framework::standard::{
        macros::command,
        CommandResult
    }
};
use rand::Rng;

fn get_rand(v: &i32) -> i32 {
    // Simple function to get the random number
    // created as the Thread interfered with the macro
    let mut rng = rand::thread_rng();
    let result = rng.gen_range(1..*v);

    // Returning the randomized result
    return result;
}

#[command]
async fn d4(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&4));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d4.");
                e.description("So, now, you face the consequences of this roll.");
                e.field("A dice falls from the sky, as you pray to the Murgi Lords.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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

#[command]
async fn d6(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&6));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d6.");
                e.description("So, now, you face the consequences of this roll.");
                e.field("A dice falls from the sky, as you pray to the Murgi Lords.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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

#[command]
async fn d8(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&8));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d8.");
                e.description("So, now, you face the consequences of this roll.");
                e.field("A dice falls from the sky, as you pray to the Murgi Lords.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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

#[command]
async fn d10(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&10));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d10.");
                e.description("So, now, you face the consequences of this roll.");
                e.field("A dice falls from the sky, as you pray to the Murgi Lords.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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

#[command]
async fn d12(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&12));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d12.");
                e.description("So, now, you face the consequences of this roll.");
                e.field("A dice falls from the sky, as you pray to the Murgi Lords.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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

#[command]
async fn d20(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&20));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d20.");
                e.description("So, now, you face the consequences of this roll.");
                e.field("A dice falls from the sky, as you pray to the Murgi Lords.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
