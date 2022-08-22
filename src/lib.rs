#![allow(unused)] // silence unused warning

use serde::{Deserialize, Serialize};

// Games struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Games {
    title: String,
    img: String,
    main: String,
    extra: String,
    completionist: String,
}

pub fn search(game: String) -> Vec<Games> {
    // Get the html from the website
    let games_found = howlongtobeat(game.to_string())
        .map_err(|err| println!("{:?}", err))
        .unwrap();

    // Parse the html into a fragment
    let document = scraper::Html::parse_document(&games_found);

    // Select the elements we want for images
    let mut img_vector = vec![];

    let img_selector = scraper::Selector::parse("div.search_list_image>a>img").unwrap();
    let img = document
        .select(&img_selector)
        .map(|x| x.value().attr("src").unwrap().to_string());

    img.zip(1..101).for_each(|(item, _)| img_vector.push(item));

    // Select the elements we want for titles
    let mut titles_vector = vec![];

    let title_selector = scraper::Selector::parse(".shadow_text>a").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles
        .zip(1..101)
        .for_each(|(item, _)| titles_vector.push(item));

    // Select the elements we want for Main Story, Main + Extra, Completionist
    let mut time_vector = vec![];

    let time_selector = scraper::Selector::parse(".search_list_tidbit").unwrap();
    let times = document.select(&time_selector).map(|x| x.inner_html());

    times.zip(0..401).for_each(|(item, _)| {
        if item == "Main Story" || item == "Main + Extra" || item == "Completionist" {
        } else {
            if item == "" {
                time_vector.push("--".to_string());
            } else {
                if item.contains("Hours") {
                    time_vector.push(item.replace(" Hours ", ""));
                } else {
                    time_vector.push("--".to_string());
                }
            }
        }
    });
    // Vector for Games struct
    let mut games = vec![];

    // Create vectors of vectors for each game time
    let time_chunks: Vec<Vec<String>> = time_vector.chunks(3).map(|s| s.into()).collect();
    // println!("{:#?}", time_chunks);

    // Create vector of Games structs
    // verified time_chunks in the future (i think it may couse a lost of data)
    for i in 0..time_chunks.len() {
        let game = Games {
            title: titles_vector[i].to_string(),
            img: String::from("https://howlongtobeat.com") + img_vector[i].to_string().as_str(),
            main: time_chunks[i][0].to_string(),
            extra: time_chunks[i][1].to_string(),
            completionist: time_chunks[i][2].to_string(),
        };

        // Serializes and pushes the game struct into the vector of games
        // let serialized = serde_json::to_string(&game).unwrap().to_string();
        // println!("{:#?}", game);

        // games.push(game);
        games.push(game);
    }

    // Return the vector of games
    return games;
}

#[tokio::main]
async fn howlongtobeat(game: String) -> Result<String, Box<dyn std::error::Error>> {
    // Post request to howlongtobeat
    let response = reqwest::Client::new()
        .post("https://howlongtobeat.com/search_results?page=1")
        .form(&[
            ("queryString", game.to_string()),
            ("t", "games".to_string()),
            ("sorthead", "popular".to_string()),
            ("sortd", "Normal Order".to_string()),
            ("plat", "".to_string()),
            ("length_type", "main".to_string()),
            ("length_min", "".to_string()),
            ("length_max", "".to_string()),
            ("detail", "0".to_string()),
            ("v", "".to_string()),
            ("f", "".to_string()),
            ("g", "".to_string()),
            ("randomize", "0".to_string()),
        ])
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36")
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("If-None-Match", "wyzzy")
        .header("Accept", "*/*")
        .header("origin", "https://howlongtobeat.com")
        .header("referer", "https://howlongtobeat.com")
        .send()
        .await?;

    // Get response from howlongtobeat
    let res_body = response.text().await?;

    // Return response from howlongtobeat
    return Ok(res_body);
}
