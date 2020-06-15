

use library;

fn main() {
    let key = library::buy_song_mod::read_private_key();
    println!("private_key: \n{}\n",key);

    let lyrics = library::read_lyrics_mod::read_song();
    println!("song lyrics: \n{}\n",lyrics);

}