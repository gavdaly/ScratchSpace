use anyhow::Result;
use scraper::{Html, Selector};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PhoneType {
    Cell,
    Landline,
    Unknown,
}

#[derive(Debug)]
pub struct Phone {
    number: String,
    phone_type: PhoneType,
}

impl Phone {
    pub async fn new(number: &str) -> Result<Phone> {
        let number = parse_phone_number(number);
        let phone_type = get_line_type(&number).await?;
        Ok(Self { number, phone_type })
    }
    pub fn line_type(&self) -> PhoneType {
        self.phone_type
    }
    pub fn is_cell_phone(&self) -> bool {
        self.phone_type == PhoneType::Cell
    }
}

fn uri(n: &str) -> Result<String> {
    if n.len() != 10 {
        return Err(anyhow::Error::msg("Invalid Phone Number"));
    }

    let area = &n[0..3];
    let three = &n[3..6];
    let four = &n[6..];
    Ok(format!(
        "https://canada411.yellowpages.ca/fs/1-{area}-{three}-{four}/"
    ))
}

async fn get_line_type(number: &str) -> Result<PhoneType> {
    let number = match fetch_phone_number(number).await {
        Ok(n) => n,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    };

    let doc = Html::parse_document(&number);
    let selector =
        Selector::parse("#ypgBody > div.page__container.page__container--full > div > div.fs__root > div.fs__conainer > div.fs__content > div:nth-child(3) > ul > li:nth-child(2)").expect("Error paring CSS Selector");

    let val = doc
        .select(&selector)
        .map(|e| e.inner_html())
        .collect::<String>();

    if val.contains("Cell Number") {
        return Ok(PhoneType::Cell);
    }
    if val.contains("Landline") {
        return Ok(PhoneType::Landline);
    }
    Ok(PhoneType::Unknown)
}

fn parse_phone_number(number: &str) -> String {
    let mut first_digit = true;
    let result = number.chars().fold(vec![], |mut acc, c| {
        if c.is_ascii_digit() {
            if first_digit && c == '1' {
                return acc;
            }
            first_digit = false;
            acc.push(c);
        }
        acc
    });
    if result.len() != 10 {
        return "error".into();
    }

    result.iter().collect()
}

async fn fetch_phone_number(number: &str) -> Result<String, surf::Error> {
    let uri = uri(number)?;
    let body = surf::get(uri).await?.body_string().await?;
    Ok(body)
}

impl From<PhoneType> for String {
    fn from(val: PhoneType) -> Self {
        match val {
            PhoneType::Cell => "Cell".into(),
            PhoneType::Landline => "Landline".into(),
            PhoneType::Unknown => "Unknown".into(),
        }
    }
}

impl std::fmt::Display for PhoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_line_type() {
        let mobile =
            std::env::var("MOBILE_PHONE_NUMBER").expect("expect MOBILE_PHONE_NUMBER to be set");
        let phone = Phone::new(&mobile).await.unwrap();
        assert_eq!(phone.line_type(), PhoneType::Cell);
    }

    #[async_std::test]
    async fn test_is_cell_phone() {
        let mobile =
            std::env::var("MOBILE_PHONE_NUMBER").expect("expect MOBILE_PHONE_NUMBER to be set");
        let phone = Phone::new(&mobile).await.unwrap();
        assert!(phone.is_cell_phone());
    }

    #[test]
    fn test_uri_valid() {
        let result = uri("1234567890").unwrap();
        assert_eq!(
            result,
            "https://canada411.yellowpages.ca/fs/1-123-456-7890/"
        );
    }

    #[test]
    fn test_uri_invalid() {
        let result = uri("12345");
        assert!(result.is_err());
    }
}
