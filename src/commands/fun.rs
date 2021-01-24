use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

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
	msg.channel_id.say(ctx, args.message()).await?;
	Ok(())
}