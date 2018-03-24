//! Decode File and Title parts from simple playlist PLS files

use std::collections::HashMap;

pub struct PlaylistItem {
    pub title: String,
    pub url: String,
}

pub fn decode(content: &str) -> Vec<PlaylistItem> {
    let lines = content.lines();
    let mut list = vec![];
    let mut found_pls = false;
    let mut map_urls = HashMap::new();
    let mut map_title = HashMap::new();
    for line in lines {
        if line.starts_with("#") {
            continue;
        } else if line.trim() == "[playlist]" {
            found_pls = true;
        } else {
            if found_pls {
                if line.starts_with("File") {
                    let idend = line.find('=').expect("");
                    let (key, value) = line.split_at(idend);
                    let id: u32 = key[4..idend].parse().expect("");
                    let (_, url) = value.split_at(1);
                    map_urls.insert(id, url);
                } else if line.starts_with("Title") {
                    let idend = line.find('=').expect("");
                    let (key, value) = line.split_at(idend);
                    let id: u32 = key[5..idend].parse().expect("");
                    let (_, title) = value.split_at(1);
                    map_title.insert(id, title);
                }
            }
        }
    }

    for (key, value) in map_urls {
        let title = map_title.get(&key).unwrap_or(&"");
        list.push(PlaylistItem {
            title: String::from(*title),
            url: String::from(value),
        });
    }

    list
}
