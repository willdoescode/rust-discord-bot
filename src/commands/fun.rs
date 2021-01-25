use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};
use serenity::utils::Colour;
use rand::Rng;

#[command]
async fn pfp(ctx: &Context, msg: &Message) -> CommandResult {
	msg.react(ctx, ReactionType::Unicode("👍".parse().unwrap())).await?;
	if msg.mentions.len() > 0 {
		msg.channel_id.send_message(ctx, |m| {
			m.embed(|e| {
				e.title(format!("{} avatar", msg.author.name));
				e.color(Colour::from_rgb(
					rand::thread_rng().gen_range(1..255),
					rand::thread_rng().gen_range(1..255),
					rand::thread_rng().gen_range(1..255)
				));
				e.image(
					msg.mentions
							.first()
							.unwrap()
							.avatar_url()
							.unwrap_or(
								"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fnetflixroulette.files.wordpress.com%2F2013%2F01%2Fimage-not-found.gif&f=1&nofb=1"
										.parse()
										.unwrap()
							)
				);
				e
			});
			m
		}).await?;
	} else {
		msg.channel_id.send_message(ctx, |m| {
			m.embed(|e| {
				e.title(format!("{} avatar", msg.author.name));
				e.color(Colour::from_rgb(
					rand::thread_rng().gen_range(1..255),
					rand::thread_rng().gen_range(1..255),
					rand::thread_rng().gen_range(1..255)
				));
				e.image(
					msg
							.author
							.avatar_url()
							.unwrap_or(
								"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fnetflixroulette.files.wordpress.com%2F2013%2F01%2Fimage-not-found.gif&f=1&nofb=1"
										.parse()
										.unwrap()
							)
				);
				e
			});
			m
		}).await?;
	}
	Ok(())
}

#[command]
async fn echo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if !msg.mention_everyone {
		msg.reply(ctx, format!("{}: {}", msg.author.name, args.message())).await?;
	}
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
						if first.clone().unwrap() > second.clone().unwrap() {
							msg.reply(ctx, "First num must be smaller than second").await?;
						} else {
							let num = rand::thread_rng().gen_range(first.unwrap_or(1)..second.unwrap_or(7));
							msg.reply(ctx, num).await?;
						}
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