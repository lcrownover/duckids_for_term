use clap::Parser;
use rayon::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// Get a list of DuckIDs given a Banner Term Code and Class Registration Number (CRN)
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Term Code
    #[arg(short, long)]
    term_code: String,

    /// CRN
    #[arg(short, long)]
    crn: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BannerRosterUser {
    #[serde(rename = "bannerID")]
    banner_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BannerRosterResponse {
    #[serde(rename = "termCode")]
    term_code: String,

    crn: String,

    #[serde(rename = "courseTitle")]
    course_title: String,

    #[serde(rename = "subjectCode")]
    subject_code: String,

    #[serde(rename = "courseNumber")]
    course_number: String,

    instructors: Vec<BannerRosterUser>,
    students: Vec<BannerRosterUser>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BannerDuckIDResponse {
    message: String,

    data: BannerDuckIDData,

    #[serde(rename = "statusCode")]
    status_code: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BannerDuckIDData {
    #[serde(rename = "bannerID")]
    banner_id: String,

    #[serde(rename = "duckID")]
    duck_id: String,
}

fn roster_url(term_code: &str, crn: &str) -> String {
    format!(
        "https://api.uoregon.edu/course/v2/roster/{}/{}",
        term_code, crn
    )
}

fn duckid_url(banner_id: &str) -> String {
    format!("https://api.uoregon.edu/person/uo/duckid/{}", banner_id)
}

fn get_roster(
    term_code: &str,
    crn: &str,
    api_key: &str,
) -> Result<BannerRosterResponse, Box<dyn std::error::Error>> {
    let url = roster_url(term_code, crn);

    let client = Client::new();
    let body = client
        .get(url)
        .header("Ocp-Apim-Subscription-Key", api_key)
        .send()?
        .text()?;

    Ok(serde_json::from_str::<BannerRosterResponse>(&body)?)
}

fn get_duckid_from_banner_id(
    banner_id: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = duckid_url(banner_id);
    let client = Client::new();
    let body = client
        .get(url)
        .header("Ocp-Apim-Subscription-Key", api_key)
        .send()?
        .text()?;
    let resp = serde_json::from_str::<BannerDuckIDResponse>(&body)?;
    Ok(resp.data.duck_id)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let api_key =
        env::var("BANNER_API_KEY").expect("BANNER_API_KEY environment variable is not set");

    let roster = get_roster(&args.term_code, &args.crn, &api_key)?;

    let student_ids: Vec<String> = roster
        .students
        .iter()
        .map(|s| s.banner_id.clone())
        .collect::<Vec<String>>();
    let instructor_ids: Vec<String> = roster
        .instructors
        .iter()
        .map(|s| s.banner_id.clone())
        .collect::<Vec<String>>();

    let banner_ids = [student_ids, instructor_ids].concat();

    let duckids: Vec<String> = banner_ids
        .par_iter()
        .map(|b| get_duckid_from_banner_id(&b, &api_key).expect("failed to get duckid"))
        .collect::<Vec<String>>();

    for duckid in duckids {
        println!("{}", duckid);
    }

    Ok(())
}
