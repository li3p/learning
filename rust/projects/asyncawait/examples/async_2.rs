use futures::executor::block_on;

struct Song {
    title: String,
    artist: String,
}

async fn learn_song() -> Song {
    Song {
        title: String::from("A song"),
        artist: String::from("An artist"),
    }
}

async fn sing_song(song: Song) {
    println!("Singing {} of {}", song.title, song.artist);
}

async fn dance() {
    println!("Dancing!");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let dance = dance();
    let sing = learn_and_sing();

    futures::join!(dance, sing);
}

fn main() {
    block_on(async_main());
}
