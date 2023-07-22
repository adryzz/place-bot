use std::fmt::Display;

use anyhow::anyhow;
use reqwest::Client;
use serde::Serialize;

const OPERATION_NAME: &str = "setPixel";
const ACTION_NAME: &str = "r/replace:set_pixel";
const QUERY: &str = "mutation setPixel($input: ActInput!) {\n  act(input: $input) {\n    data {\n      ... on BasicMessage {\n        id\n        data {\n          ... on GetUserCooldownResponseMessageData {\n            nextAvailablePixelTimestamp\n            __typename\n          }\n          ... on SetPixelResponseMessageData {\n            timestamp\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n";

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PixelQuery {
    #[serde(rename = "operationName")]
    operation_name: &'static str,
    variables: PixelQueryVariables,
    query: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PixelQueryVariables {
    input: PixelQueryVariablesInput,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PixelQueryVariablesInput {
    #[serde(rename = "actionName")]
    action_name: &'static str,

    #[serde(rename = "PixelMessageData")]
    pixel_message_data: PixelMessageData,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PixelMessageData {
    coordinate: Coordinates,
    #[serde(rename = "colorIndex")]
    color_index: i32,
    #[serde(rename = "canvasIndex")]
    canvas_index: i32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    pub fn new(mut x: i32, mut y: i32) -> anyhow::Result<Self> {
        x += 1500;
        y += 1000;

        x %= 1000;
        y %= 1000;

        Ok(Self { x, y })
    }
}

impl PixelQuery {
    pub fn new(x: i32, y: i32, color: Color) -> anyhow::Result<Self> {
        Ok(Self {
            operation_name: OPERATION_NAME,
            variables: PixelQueryVariables {
                input: PixelQueryVariablesInput {
                    action_name: ACTION_NAME,
                    pixel_message_data: PixelMessageData {
                        coordinate: Coordinates::new(x, y)?,
                        color_index: color as i32,
                        canvas_index: coordinates_to_canvas(x, y)?,
                    },
                },
            },
            query: QUERY,
        })
    }
}

fn coordinates_to_canvas(mut x: i32, mut y: i32) -> anyhow::Result<i32> {
    x += 1500;
    y += 1000;

    if x <= 1000 && y <= 1000 {
        return Ok(0);
    }
    if x <= 2000 && y <= 1000 {
        return Ok(1);
    }
    if x <= 3000 && y <= 1000 {
        return Ok(2);
    }
    if x <= 1000 && y <= 2000 {
        return Ok(3);
    }
    if x <= 2000 && y <= 2000 {
        return Ok(4);
    }
    if x <= 3000 && y <= 2000 {
        return Ok(5);
    }

    Err(anyhow!("Invalid coordinates ({}, {})", x, y))
}

pub async fn make_query(
    client: &Client,
    x: i32,
    y: i32,
    color: Color,
    bearer: &str,
) -> anyhow::Result<()> {
    let _ = client
        .post("https://gql-realtime-2.reddit.com/query")
        .bearer_auth(bearer)
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept-Language", "en-US;en;q=0.5")
        .header("apollographql-client-name", "garlic-bread")
        .header("apollographql-client-version", "0.0.1")
        .header("Origin", "https://garlic-bread.reddit.com")
        .header("Referer", "https://garlic-bread.reddit.com")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-site")
        .header("TE", "trailers")
        .header("Connection", "keep-alive")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0",
        )
        .json(&PixelQuery::new(x, y, color)?)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}


#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, Display)]
pub enum Color {
    Red = 2,
    Orange = 3,
    Yellow = 4,
    DarkGreen = 6,
    LightGreen = 8,
    DarkBlue = 12,
    Blue = 13,
    LightBlue = 14,
    DarkPurple = 17,
    Purple = 18,
    LightPink = 23,
    Brown = 25,
    Black = 27,
    Gray = 29,
    LightGray = 30,
    White = 31,
}
