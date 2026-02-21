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
    print_dedia(audio_book);
}
