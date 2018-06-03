use serenity::framework::standard::CommandError;
use serenity::model::guild::Member;

command!(ban(_context, message, args) {
    let guild = match message.guild() { // Check to make sure they aren't using ~ban in DMs
        Some(val) => val.read().clone(),
        None => {
            message.reply("Command must be issued from a guild.").expect("Error sending message ");
            return Err(CommandError::from("Command not issued from guild."));
        },
    };

    let mut member: Option<Member> = None;
    if message.mentions.len() > 0 { // Check if the target is mentioned
        if let Ok(user) = guild.member(&message.mentions[0]) {
            member = Some(user);
        }
    }

    let mut reason: String = "".to_string();
    if args.len() == 2 {
        if member.is_none() { // Check if the target is named
            let search = args.single::<String>().unwrap();
            let members = guild.members_containing(&search, true, true);

            member = match members.first() { // Get first result in search
                Some(val) => Some(val.clone().clone()), // Why does this work?
                None => None,
            }
        }
        else { // Discard first argument if it's unuseful
            args.skip();
        }
        reason = args.single::<String>().unwrap(); // Parse second arg as string and use as reason
    }

    let options: (u8, &str) = (7, &reason); // Remove 7 days of messages and supply reason
    match member { // Ban the user if applicable
        Some(val) => {
            message.reply(&format!("Banned <@{}>", val.user.read().id)).expect("Error sending message");
            val.ban(&options).expect("Error banning member.")
        },
        None => {
            message.reply("Could not find member from input.").expect("Error sending message");
            return Err(CommandError::from("Member not found."));
        }
    }
});
