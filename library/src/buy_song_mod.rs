
cfg_if::cfg_if! {
    if #[cfg(feature = "unsafe_feature_read_private_key")] {
        pub fn read_private_key()->String{
            let path = "sample_data/private_keys/my_private_key.txt";
            // std::fs should be unsafe !
            // therefore the code must be under a namespace of "unsafe features"
            let file_content = std::fs::read_to_string(path).unwrap();
            //return
            file_content
        }
    }  else {
        pub fn read_private_key()->String {"no feature, no access.".to_string()}
    }
}