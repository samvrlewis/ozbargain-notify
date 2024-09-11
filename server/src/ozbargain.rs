use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use thiserror::Error;

use crate::Deal;

#[derive(Error, Debug)]
pub enum ScrapeError {
    #[error("Failed to perform HTTP request")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to parse HTML document")]
    ParseError,

    #[error("Failed to find required element for {0}")]
    ElementNotFound(&'static str),

    #[error("Failed to extract attribute {0}")]
    AttributeNotFound(&'static str),

    #[error("Failed to parse date")]
    DateParseError,

    #[error("Failed to parse votes as unsigned integer")]
    VotesParseError,
}

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap();
}

async fn fetch_page(url: &str) -> Result<String, ScrapeError> {
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

fn parse_title(deal: &scraper::ElementRef) -> Result<String, ScrapeError> {
    let title_selector = Selector::parse("h2").map_err(|_| ScrapeError::ParseError)?;
    let title = deal
        .select(&title_selector)
        .next()
        .ok_or(ScrapeError::ElementNotFound("title"))?
        .text()
        .collect::<Vec<_>>()
        .concat();

    Ok(title)
}

fn parse_votes(deal: &scraper::ElementRef) -> Result<u32, ScrapeError> {
    let votes_selector = Selector::parse(".voteup > span").map_err(|_| ScrapeError::ParseError)?;
    let vote_element = deal
        .select(&votes_selector)
        .next()
        .ok_or(ScrapeError::ElementNotFound("votes"))?;

    let votes_text = vote_element.text().collect::<String>();
    votes_text
        .parse::<u32>()
        .map_err(|_| ScrapeError::VotesParseError)
}

fn parse_id(deal: &scraper::ElementRef) -> Result<String, ScrapeError> {
    let id_selector = Selector::parse(".n-deal").map_err(|_| ScrapeError::ParseError)?;
    let id = deal
        .select(&id_selector)
        .next()
        .ok_or(ScrapeError::ElementNotFound("id"))?
        .value()
        .attr("data-nid")
        .ok_or(ScrapeError::AttributeNotFound("data-nid"))?
        .to_string();

    Ok(id)
}

fn parse_date(deal: &scraper::ElementRef) -> Result<Option<String>, ScrapeError> {
    let date_selector = Selector::parse(".submitted").map_err(|_| ScrapeError::ParseError)?;
    let date_html = deal
        .select(&date_selector)
        .next()
        .ok_or(ScrapeError::ElementNotFound("date"))?
        .inner_html();

    let date = DATE_REGEX.find(&date_html).map(|m| m.as_str().to_string());

    Ok(date)
}

fn parse_deals(document: &Html) -> Vec<Result<Deal, ScrapeError>> {
    let deal_selector = Selector::parse(".node-ozbdeal").expect("Failed to parse selector");

    document
        .select(&deal_selector)
        .map(|deal| {
            Ok(Deal {
                title: parse_title(&deal)?,
                votes: parse_votes(&deal)?,
                id: parse_id(&deal)?,
                date: parse_date(&deal)?,
            })
        })
        .collect()
}

pub async fn fetch_deals() -> Result<Vec<Deal>, ScrapeError> {
    let url = "http://www.ozbargain.com.au";

    let response = fetch_page(url).await?;
    let document = Html::parse_document(&response);

    let deals: Vec<Deal> = parse_deals(&document)
        .into_iter()
        .filter_map(|result| match result {
            Ok(deal) => Some(deal),
            Err(e) => {
                error!("Error parsing deal: {e}");
                None
            }
        })
        .collect();

    Ok(deals)
}
