cfg_if::cfg_if! {
    if #[cfg(feature = "unsafe_feature_read_song")] {
        pub fn read_song()->String{
            let file_content;
            /// every unsafe should have a doc-comment, to describe what it does
            /// this unsafe reads from "sample_data/songs/basket_case_green_day.txt"
            /// one idea how to force the existence of "unsafe feature":
            ///  unsafe("unsafe_feature_read_song"){...}
            unsafe{
                let path = "sample_data/songs/basket_case_green_day.txt";
                // std::fs should be "unsafe"
                // therefore the code must be under a namespace of "unsafe features"
                file_content = std::fs::read_to_string(path).unwrap();
            }
            //return
            file_content
        }
}  else {
    /// safe alternative
    pub fn read_song()->String {"no feature, no access.".to_string()}
}
}