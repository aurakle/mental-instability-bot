use poise::CreateReply;
use regex::Regex;

use crate::
    commands::{Context, Error}
;
use std::fmt::Write;

macro_rules! check {
    ($arg:expr,$regex:expr) => {{
        let regex = Regex::new($regex).expect("Regex error lmao");
        regex.is_match($arg)
    }};
}

/// Get the current status of the yarn cache
#[allow(irrefutable_let_patterns)]
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub(crate) async fn cache_status(ctx: Context<'_>) -> Result<(), Error> {
    // This is kinda stupid, but it stops that macro from breaking, so w/e
    if let cache = ctx.serenity_context()
        .data
        .write()
        .await
        .get_mut::<crate::MappingsCacheKey>()
        .expect("No mappings cache?") 
    {
        let keys = cache.cached_keys();
        let mut output = format!(
            "Currently caching the mappings for {} Minecraft versions.",
            keys.len()
        );
        for ele in keys {
            write!(output, "\n- `{}` {} hits", ele, cache.get_hits(ele))?;
        }

        ctx.send(CreateReply::default().content(output).ephemeral(false))
            .await?;
    }
    Ok(())
}

/// Get the yarn name of an obfuscated class, method or field
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub(crate) async fn yarn(
    ctx: Context<'_>,
    #[description = "The obfuscated class, method or field name"] name: String,
    #[description = "The relevant Minecraft version"] mc_version: String,
) -> Result<(), Error> {
    if let Some(mappings) = ctx.serenity_context()
        .data
        .write()
        .await
        .get_mut::<crate::MappingsCacheKey>()
        .expect("No mappings cache?")
        .get_or_download(&mc_version)
        .await
        .map_err(|err| {
            println!("{}", err);
            err
        })?
    {
        let result = if check!(&name, r"^class_[0-9]+$") {
            mappings.full_classes.get(&name)
        } else if check!(&name, r"^method_[0-9]+$") {
            mappings.methods.get(&name)
        } else if check!(&name, r"^field_[0-9]+$") {
            mappings.fields.get(&name)
        } else {
            None
        };

        if let Some(yarn_name) = result {
            ctx.send(
                CreateReply::default()
                    .content(format!(
                        "The yarn name for `{name}` in Minecraft `{mc_version}` is `{yarn_name}`."
                    ))
                    .ephemeral(false),
            )
            .await?;
        } else {
            ctx.send(
                CreateReply::default()
                    .content(format!(
                        "`{name}` does not exist in Minecraft `{mc_version}`."
                    ))
                    .ephemeral(true),
            )
            .await?;
        }
    } else {
        ctx.send(
            CreateReply::default()
                .content("Could not find any mappings for that Minecraft version.")
                .ephemeral(true),
        )
        .await?;
    }

    Ok(())
}
