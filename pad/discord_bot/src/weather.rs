use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug)]
pub struct CouldNotFindLocation {
    place: String
}

impl Display for CouldNofFindLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find location '{}'", self.place)
    }
}

impl std::error::Error for CouldNotFindLocation {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    key: String,
    localized_name: String,
    country: Country,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.localized_name, self.country.id)
    }
}

#[derive(Deserialize, Debug)]
pub struct Country {
    #[serde(alias = "ID")]
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = PascalCase)]
pub struct Forcast {
    pub headline: Headline,
}

#[derive(Deserialize, Debug)]
pub struct Headline {
    #[serde(alias = "Text")]
    pub overview: String,
}

pub async fn get_forecast(place: &str, api_key: &str, client: &Client) -> Result<(Location, Forecast), Box<dyn std::error::Error>> {
    const LOCATION_REQUEST: &str = "http://dataservice.accuweather.com/locations/v1/cities/search";
    const DAY_REQUEST: &str = "http://dataservice.accuweather.com/forecasts/v1/daily/1day/";

    let url = format!("{}?apikey={}&q={}", LOCATION_REQUEST, api_key, place);

    let request = client.get(url).build().unwrap();

    let resp = client.execute(request).await?.json::<Vec<Location>>().await?;

    let first_location = resp.into_iter().next().ok_or_else(|| CouldNotFindLocation {place: place.to_owned()})?;

    let url = format!("{}{}?apikey={}", DAY_REQUEST, first_location.key, api_key);

    let request = client.get(url).build().unwrap();

    let forecast = client.execute(request).await?.json::<Forecast>().await?;

    Ok((first_location, forecast))
}
