use library;

fn main() {
    
    let lyrics = library::read_lyrics_mod::read_song();
    println!("song lyrics: \n{}\n",lyrics);

    let key = library::buy_song_mod::buy_song("ask me later");
    println!("buy song: \n{}\n",key);

}