use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};
use rand::Rng;

#[command]
async fn pfp(ctx: &Context, msg: &Message) -> CommandResult {
	msg.react(ctx, ReactionType::Unicode("👍".parse().unwrap())).await?;
	if msg.mentions.len() > 0 {
		msg.channel_id.say(
			ctx,
			msg.mentions[0].avatar_url().unwrap_or(
				format!("Could not get profile picture of {}", msg.mentions[0].name
				))).await?;
	} else {
		msg.channel_id.say(ctx, msg.author.avatar_url().unwrap()).await?;
	}
	Ok(())
}

#[command]
async fn echo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	msg.reply(ctx, format!("{}: {}", msg.author.name, args.message())).await?;
	Ok(())
}

#[command]
async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if !args.is_empty() {
		let message_contents = args.message().split(" ").collect::<Vec<&str>>();
		match message_contents[0] {
			"range" => {
				if message_contents.get(1).is_some() && message_contents.get(2).is_some() {
					let first = message_contents[1].parse::<i32>();
					let second = message_contents[2].parse::<i32>();
					if first.is_ok() && second.is_ok() {
						let num = rand::thread_rng().gen_range(first.unwrap_or(1)..second.unwrap_or(7));
						msg.reply(ctx, num).await?;
					} else {
						msg.reply(ctx, "Invalid digits").await?;
					}
				} else {
					msg.reply(ctx, "Must include range like .roll range 0 9").await?;
				}
			}
			_ => {
				msg.reply(ctx, "Invalid roll option").await?;
				()
			}
		}
	} else {
		let num = rand::thread_rng().gen_range(1..7);
		msg.reply(ctx, num).await?;
	}

	Ok(())
}