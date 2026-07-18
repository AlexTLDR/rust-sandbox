trait Playable {
    fn play(&self) -> String;
}

struct AudioBook {
    title: String,
    author: String,
}

struct VideoGame {
    name: String,
    platform: String,
}

struct Metadata<'a> {
    description: &'a str,
}

impl<'a> Metadata<'a> {
    fn describe(&self) {
        println!("Description is {}", self.description)
    }
}

impl Playable for AudioBook {
    fn play(&self) -> String {
        format!(
            "Now playing book {} written by {}...",
            self.title, self.author
        )
    }
}

impl Playable for VideoGame {
    fn play(&self) -> String {
        format!("Launching game {} on {}...", self.name, self.platform)
    }
}

fn consume_media<T: Playable>(media: &T) {
    println!("{}", media.play())
}

fn main() {
    let book = AudioBook {
        title: String::from("The Rust Programming Handbook"),
        author: String::from("Francesco Ciulla"),
    };
    let game = VideoGame {
        name: String::from("Football Manager"),
        platform: String::from("PC"),
    };
    consume_media(&book);
    consume_media(&game);

    let meta = Metadata {
        description: "Football Manager is one of the greatest games of all time",
    };
    meta.describe();
}
