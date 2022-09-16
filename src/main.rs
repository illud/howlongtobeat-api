fn main(){
    let games = howlongtobeat::search("Elden Ring".to_string());
    println!("{:#?}", games);
}