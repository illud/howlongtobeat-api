use howlongtobeat;

#[test]
fn test_search() {
    let games = howlongtobeat::search("Elden Ring".to_string());
    println!("{:?}", games);
    assert_eq!(games.len(), 1);
}
