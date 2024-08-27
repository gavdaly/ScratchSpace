struct Bot {
    weather_api_key: String,
    client: reqwest::Client,
    discord_guild_id: guildId,
}

#[shuttle_service::main]
async fn serenity(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_service::ShuttleSerenity {
    let token = secret_token.get("DISCORD_TOKEN").context("'DISCORD_TOKEN' was not found")?;
    let weather_api_key = secret_store.get("WEATHER_API_KEY").context("'WEATHER_API_KEY' was not found")?;
    let discord_guild_id = secret_store.get("DISCORD_GUILD_ID").context("'DISCORD_GUILD_ID' was not found")?;
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(&token, intents).envent_handler(Bot {weather_api_key, client: reqwest::Client::new(), discrod_guild_id: GuildId(discord_guild_id.parse().unwrap())}).await.expect("Err creating client");
    Ok(client)
}
