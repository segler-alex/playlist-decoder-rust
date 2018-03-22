pub struct PlaylistItem {
    pub url: String,
}

pub fn decode_playlist(content: &str) -> Vec<PlaylistItem> {
    let lines = content.lines();
    let mut list = vec![];
    for line in lines {
        if line.starts_with("#") {
            continue;
        }

        list.push(PlaylistItem{url:String::from(line)});
    }
    list
}