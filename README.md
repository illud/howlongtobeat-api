# Howlongtobeat API

## About & Credits

[How long to beat](https://howlongtobeat.com/) provides information and data about games and how long it will take to finish them.

This library is a simple wrapper api to fetch data from [How long to beat](https://howlongtobeat.com/) (search and detail).
It is an awesome website and a great service, also heavily living from community data. Please check the website and [support](https://howlongtobeat.com/donate.php) if you like what they are doing.

## Usage

### Install the dependency

```rust
howlongtobeat = "0.4.0"
```

### Use in code

#### Add imports


```rust
use howlongtobeat::search;
```


#### Searching for a game

```rust
let games_found = search("Elden Ring".to_string());
println!("{:#?}", games_found);
```

* Search response example:

```rust
[
    Games {
        title: "Elden Ring",
        img: "https://howlongtobeat.com/games/68151_Elden_Ring.jpg",
        main: "52",
        extra: "98",
        completionist: "131",
    },
    Games {
        title: "Elden Ring GB",
        img: "https://howlongtobeat.com/games/108888_Elden_Ring_GB.jpg",
        main: "--",
        extra: "--",
        completionist: "--",
    },
]
```

### Missing features
    Single-Player
    Solo
    Co-Op
    Vs.

### Why missing features
    To get the hours to complete a online game is really hard becouse it depends on how long you will play it or how many updates will the game have in the future. Not even in howlongtobeat page is accurated for example take a look at (Valorant) it says 23½ Hours ive play it and it take way more than that, there are many online games that shows the wrong time.

## License

DO WHAT THE FUCK YOU WANT