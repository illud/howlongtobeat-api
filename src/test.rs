use super::*;

#[test]
fn test_search() {
    let games: Vec<Game> = search("Elden Ring".to_string());

    let expected: [Game; 2] = [
        Game {
            image: String::from("https://howlongtobeat.com/games/68151_Elden_Ring.jpg".to_owned()),
            title: String::from("Elden Ring"),
            main: String::from("54h 22m"),
            extra: String::from("98h 55m"),
            completionist: String::from("132h 19m"),
        },
        Game {
            image: String::from(
                "https://howlongtobeat.com/games/108888_Elden_Ring_GB.jpg".to_owned(),
            ),
            title: String::from("Elden Ring GB"),
            main: String::from("0h 19m"),
            extra: String::from("0h 29m"),
            completionist: String::from("0h 0m"),
        },
    ];

    assert_eq!(games, expected);
}
