use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};
use std::fmt::Display;

struct Command {
	command: String,
	help: String
}

impl Command {
	fn new(command: &str, help: &str) -> Self {
		Self {
			command: command.parse().unwrap(),
			help: help.parse().unwrap()
		}
	}
}

#[command]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if !args.is_empty() {
		msg.reply( ctx, match args.message().trim() {
			"pfp" => Command::new(".pfp", "do .pfp <@user> to show a users pfp"),
			"help" => Command::new(".help", ".help <command> for advanced info on the command"),
			"ping" => Command::new(".ping", "See if the bot is running"),
			"roll" => Command::new(".roll", ".roll for 1-6 or .roll range <startofrange> <endofrange>"),
			_ => Command::new("Can't", "Help you there")
		}).await?;
	} else {
		let mut help_message = String::new();
		let commands = [
			Command::new(".help", "Give you some help"),
			Command::new(".pfp", "Shows you some profile pictures"),
			Command::new(".ping", "Pong!"),
			Command::new(".roll", "Get a random number")
		];
		for c in commands.iter() {
			help_message.push_str(&format!("{}", c))
		}
		msg.reply(ctx, help_message).await?;
	}
	Ok(())
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "{} => {}", self.command, self.help)
	}
}
