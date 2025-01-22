use poise::{
    serenity_prelude::{self as serenity, Timestamp},
    CreateReply,
};

use crate::{Context, Error};

#[poise::command(slash_command, guild_only)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let guild = ctx.partial_guild().await.unwrap();

    let user_member = match guild.member(ctx, user.id).await {
        Ok(member) => member,
        Err(_) => Err("Failed to fetch the selected user as a member")?,
    };

    let embed = serenity::CreateEmbed::new()
        .title(format!("{0} (`{1}`)", user.name, user.id))
        .color(serenity::Color::DARK_GREY)
        .field(
            "Joined Discord",
            serenity::FormattedTimestamp::new(
                user.created_at(),
                Some(serenity::FormattedTimestampStyle::RelativeTime),
            )
            .to_string(),
            true,
        )
        .field(
            format!("Joined {0}", guild.name),
            serenity::FormattedTimestamp::new(
                user_member.joined_at.unwrap_or_else(Timestamp::default),
                Some(serenity::FormattedTimestampStyle::RelativeTime),
            )
            .to_string(),
            true,
        )
        .image(user.avatar_url().unwrap_or(user.default_avatar_url()));

    let response = CreateReply::default().embed(embed);

    ctx.send(response).await.unwrap();

    Ok(())
}
