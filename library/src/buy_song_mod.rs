
cfg_if::cfg_if! {
    if #[cfg(feature = "unsafe_feature_read_private_key")] {
        use crate::read_private_key_mod;
        // call the fn read_private_key and buy a song
        pub fn buy_song(_dummy:&str)->String {
            let private_key = read_private_key_mod::read_private_key();
            format!("your private key is: {}", private_key)
        }
    }  else {
        // receive the encrypted string from the parent
        // forbid this library to read private_keys
        // the parent can use the private_key, because it is our code and
        // not a third party library
        pub fn buy_song(encrypted_string:&str)->String {
            format!("forbidden to read the private key. Using alternative method: {}",encrypted_string)
        }
    }
}