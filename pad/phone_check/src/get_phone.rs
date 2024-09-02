use scraper::{Html, Selector};

#[derive(Copy, Clone, Debug)]
pub enum PhoneType {
    Cell,
    Landline,
    Unknown,
}

#[derive(Debug)]
pub struct Phone {
    number: String,
}

impl Phone {
    pub fn new(number: &str) -> Phone {
        let number = parse_phone_number(number);
        Phone { number }
    }
    pub async fn line_type(&self) -> Result<PhoneType, String> {
        get_line_type(&self.number).await
    }
    pub async fn is_cell_phone(&self) -> bool {
        match self.line_type().await {
            Ok(PhoneType::Cell) => true,
            _ => false,
        }
    }
}

fn uri(n: &str) -> String {
    let area = &n[0..3];
    let three = &n[3..6];
    let four = &n[6..];
    format!("https://canada411.yellowpages.ca/fs/1-{area}-{three}-{four}/")
}

async fn get_line_type(number: &str) -> Result<PhoneType, String> {
    if number.len() != 10 {
        return Err("Invalid Phone Number".to_owned());
    }

    let number = match fetch_phone_number(number).await {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };

    let doc = Html::parse_document(&number);
    let selector =
        Selector::parse("#ypgBody > div.page__container.page__container--full > div > div.fs__root > div.fs__conainer > div.fs__content > div:nth-child(3) > ul > li:nth-child(2)").expect("Error paring CSS Selector");

    dbg!(&doc);

    let val = doc
        .select(&selector)
        .map(|e| e.inner_html())
        .collect::<String>();

    dbg!(&val);

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
    let uri = uri(number);
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
