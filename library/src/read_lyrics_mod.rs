cfg_if::cfg_if! {
    if #[cfg(feature = "unsafe_feature_read_song")] {
        pub fn read_song()->String{
            let path = "sample_data/songs/basket_case_green_day.txt";
            let file_content;
            unsafe{
                // std::fs should be "unsafe"
                // therefore the code must be under a namespace of "unsafe features"
                file_content = std::fs::read_to_string(path).unwrap();
            }
            //return
            file_content
        }
}  else {
    pub fn read_song()->String {"no feature, no access.".to_string()}
}
}