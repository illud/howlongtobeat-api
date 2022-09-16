# Howlongtobeat API

## About & Credits

[How long to beat](https://howlongtobeat.com/) provides information and data about games and how long it will take to finish them.

This library is a simple wrapper api to fetch data from [How long to beat](https://howlongtobeat.com/) (search and detail).
It is an awesome website and a great service, also heavily living from community data. Please check the website and [support](https://howlongtobeat.com/donate.php) if you like what they are doing.

## Usage

### Install the dependency

```rust
howlongtobeat = "0.5.0"
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
    Game {
        image: "https://howlongtobeat.com/games/68151_Elden_Ring.jpg",
        title: "Elden Ring",
        main: "52h 39m",
        extra: "98h 11m",
        completionist: "131h 19m",
    },
    Game {
        image: "https://howlongtobeat.com/games/108888_Elden_Ring_GB.jpg",
        title: "Elden Ring GB",
        main: "0h 21m",
        extra: "0h 29m",
        completionist: "0h 0m",
    },
]
```

## License

DO WHAT THE FUCK YOU WANT