use scraper::{Html, Selector};
use surf::Result;

#[derive(Debug)]
pub struct Phone {
    pub number: String,
}

impl Phone {
    pub fn new(number: &str) -> Phone {
        let number = parse_phone_number(&number);
        Phone { number }
    }
    pub async fn line_type(&self) -> Result<String> {
        get_line_type(&self.number).await
    }
    pub async fn is_cell_phone(&self) -> bool {
        match self.line_type().await {
            Ok(t) => t == "Cell Number".to_owned(),
            Err(_) => false,
        }
    }
}

fn uri(n: &str) -> String {
    let area = &n[0..3];
    let three = &n[3..6];
    let four = &n[6..];
    format!(
        "http://canada411.yellowpages.ca/fs/1-{}-{}-{}?what={}-{}-{}",
        area, three, four, area, three, four,
    )
}

async fn get_line_type(number: &str) -> Result<String> {
    if number.len() != 10 {
        return Ok("unknown".to_owned());
    }
    let uri = uri(number);

    let body = surf::get(uri).await?.body_string().await?;

    let doc = Html::parse_document(&body);
    let selector =
        Selector::parse(".phone__details > li:nth-child(2)").expect("Error paring CSS Selector");
    let val = doc
        .select(&selector)
        .map(|e| e.inner_html())
        .collect::<String>();

    if val.contains("Cell Number") {
        return Ok("Cell Number".to_owned());
    }
    if val.contains("Landline") {
        return Ok("Landline".to_owned());
    }
    Ok("unknown".to_owned())
}

fn parse_phone_number(number: &str) -> String {
    let mut first_digit = true;
    let result = number.chars().into_iter().fold(vec![], |mut acc, c| {
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
