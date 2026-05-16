#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

fn print_dedia(media: Media) {
    println!("{:#?}", media);
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book {} {}", title, author),
            Media::Audiobook { title } => format!("Book {}", title),
            Media::Movie { title, director } => format!("Book {} {}", title, director),
            Media::Podcast(id) => format!("Podcast: {}", id),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, idx: usize) -> MightHaveAValue {
        if self.items.len() > idx {
            MightHaveAValue::ThereIsAValue(&self.items[idx])
        } else {
            MightHaveAValue::NoValueAvailable
        }
    }
}

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

fn main() {
    let audio_book = Media::Audiobook {
        title: "An audiobook".to_string(),
    };

    let good_movie = Media::Movie {
        title: "Good Movie".to_string(),
        director: "Good Director".to_string(),
    };
    let bad_book = Media::Book {
        title: "Bad book".to_string(),
        author: "bad author".to_string(),
    };

    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", audio_book.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();
    catalog.add(audio_book);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // let item = catalog.get_by_index(40);

    // println!("{:#?}", item);
    //

    // match catalog.get_by_index(40) {
    //     MightHaveAValue::ThereIsAValue(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("No value here!")
    //     }
    // }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(40) {
        println!("Item: {:#?}", value);
    } else {
        println!("No value here!")
    }
}
