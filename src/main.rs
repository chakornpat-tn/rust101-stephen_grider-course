#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

fn print_dedia(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audio_book = Media::Audiobook {
        title: "An audiobook".to_string(),
    };

    let good_movie = Media::Movie {
        title: "Good Movie".to_string(),
        director: "Good Director".to_string(),
    };
    let bad_movie = Media::Book {
        title: "Bad Movie".to_string(),
        author: "bad author".to_string(),
    };

    print_dedia(audio_book);
    print_dedia(good_movie);
    print_dedia(bad_movie);
}
