use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct HowlongtobeatResponse<Object> {
    data: Vec<Object>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Howlongtobeat {
    pub game_image: String,
    pub game_name: String,
    pub comp_main: i32,
    pub comp_plus: i32,
    pub comp_100: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Game {
    pub image: String,
    pub title: String,
    pub main: String,
    pub extra: String,
    pub completionist: String,
}

pub fn search(game: String) -> Vec<Game> {
    let games_found: Vec<Game> = howlongtobeat(game.to_string()).unwrap();

    return games_found;
}

fn seconds_to_time(time: i32) -> String {
    let hours: i32 = time / 3600;
    let minutes: i32 = time % 3600 / 60;

    return hours.to_string() + "h " + &minutes.to_string() + "m";
}

fn line_to_words(line: &str) -> Vec<String> {
    line.split_whitespace().map(str::to_string).collect()
}

#[tokio::main]
async fn howlongtobeat(game: String) -> Result<Vec<Game>, reqwest::Error> {
    // Post request to howlongtobeat
    let response: serde_json::Value = reqwest::Client::new()
        .post("https://www.howlongtobeat.com/api/search")
        .json(&serde_json::json!(
            {
                "searchType": "games",
                "searchTerms": line_to_words(game.as_str()),
                "searchPage": 1,
                "size": 20,
                "searchOptions": {
                  "games": {
                    "userId": 0,
                    "platform": "",
                    "sortCategory": "popular",
                    "rangeCategory": "main",
                    "rangeTime": {
                      "min": 0,
                      "max": 0
                    },
                    "gameplay": {
                      "perspective": "",
                      "flow": "",
                      "genre": ""
                    },
                    "modifier": ""
                  },
                  "users": {
                    "sortCategory": "postcount"
                  },
                  "filter": "",
                  "sort": 0,
                  "randomizer": 0
                }
              }
        ))
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36")
        .header("Content-type", "application/json; charset=UTF-8")
        .header("If-None-Match", "wyzzy")
        .header("Accept", "*/*")
        .header("origin", "https://howlongtobeat.com")
        .header("referer", "https://howlongtobeat.com")
        .send()
        .await?
        .json()
        .await?;

    let json_games_found: HowlongtobeatResponse<Howlongtobeat> =
        serde_json::from_str(&response.to_string()).expect("JSON was not well-formatted");

    let mut games: Vec<Game> = vec![];

    for elem in json_games_found.data {
        games.push(Game {
            image: String::from("https://howlongtobeat.com/games/".to_owned() + &elem.game_image),
            title: String::from(elem.game_name),
            main: seconds_to_time(elem.comp_main),
            extra: seconds_to_time(elem.comp_plus),
            completionist: seconds_to_time(elem.comp_100),
        })
    }
    // Return response from howlongtobeat
    return Ok(games);
}

//test
#[cfg(test)]
mod test;
