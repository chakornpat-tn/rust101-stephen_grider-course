#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
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
        }
    }
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

    println!("{}", audio_book.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
}
