use super::{Context, Error};

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub(crate) async fn make_reminder(
    ctx: Context<'_>,
    #[description = "Make a reminder for the given time"] time: String,
    message: String
) -> Result<(), Error> {
    Ok(())
}
