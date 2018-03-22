extern crate quick_xml;

mod pls;
mod m3u;
mod asx;
mod xspf;

pub fn decode_playlist(content: &str) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    match content.to_lowercase().find("<playlist"){
        Some(_)=>{
            let xspf_items = xspf::decode_playlist(content);
            for item in xspf_items {
                list.push(item.url);
                list.push(item.identifier);
            }
        }
        None =>{
            match content.to_lowercase().find("<asx"){
                Some(_)=>{
                    let pls_items = asx::decode_playlist(content);
                    for item in pls_items {
                        list.push(item.url);
                    }
                }
                None =>{
                    match content.to_lowercase().find("[playlist]"){
                        Some(_) => {
                            let pls_items = pls::decode_playlist(content);
                            for item in pls_items {
                                list.push(item.url);
                            }
                        }
                        None => {
                            let m3u_items = m3u::decode_playlist(content);
                            for item in m3u_items {
                                list.push(item.url);
                            }
                        }
                    }
                }
            }
        }
    }
    
    list
}

#[cfg(test)]
mod tests {
    #[test]
    fn m3u() {
        use m3u;
        let items = m3u::decode_playlist("http://this.is.an.example");
        assert!(items.len() == 1);
        assert!(items[0].url == "http://this.is.an.example");
    }
}
