mod get_phone;

use tide::{log, Request, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let address = "127.0.0.1:8080";
    let mut app = tide::new();
    app.at("/:number").get(get_phone_type);
    app.at("/iscell/:number").get(is_phone_mobile);
    app.listen(address).await?;
    log::info!("listening on: {}", address);
    Ok(())
}

async fn is_phone_mobile(req: Request<()>) -> Result<String> {
    let number = req.param("number")?;
    Ok(get_phone::Phone::new(number)
        .await?
        .is_cell_phone()
        .to_string())
}

async fn get_phone_type(req: Request<()>) -> Result<String> {
    let number = req.param("number")?;
    Ok(get_phone::Phone::new(number).await?.line_type().to_string())
}
