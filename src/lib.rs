//! This is a very simple url extractor for different kinds of playlist formats: M3U, PLS, ASX, XSPF
//!
//! It is not optimized yet and does create a lot of strings on the way.

extern crate quick_xml;

mod pls;
mod m3u;
mod asx;
mod xspf;

use std::collections::HashSet;
use std::error::Error;

/// Decode playlist content string. It checks for M3U, PLS, XSPF and ASX content in the string.
/// # Example
/// ```rust
/// let list = playlist_decoder::decode(r##"<?xml version="1.0" encoding="UTF-8"?>
///    <playlist version="1" xmlns="http://xspf.org/ns/0/">
///      <trackList>
///        <track>
///          <title>Nobody Move, Nobody Get Hurt</title>
///          <creator>We Are Scientists</creator>
///          <location>file:///mp3s/titel_1.mp3</location>
///        </track>
///        <track>
///          <title>See The World</title>
///          <creator>The Kooks</creator>
///          <location>http://www.example.org/musik/world.ogg</location>
///        </track>
///      </trackList>
///    </playlist>"##).unwrap();
/// assert!(list.len() == 2, "Did not find 2 urls in example");
/// for item in list {
///     println!("{:?}", item);
/// }
/// ```
/// # Arguments
/// * `content` - A string slice containing a playlist
pub fn decode(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut set = HashSet::new();
    let content_small = content.to_lowercase();
    match content_small.find("<playlist"){
        Some(_)=>{
            let xspf_items = xspf::decode(content)?;
            for item in xspf_items {
                if item.url != "" {
                    set.insert(item.url);
                }
                if item.identifier != "" {
                    set.insert(item.identifier);
                }
            }
        }
        None =>{
            match content_small.find("<asx"){
                Some(_)=>{
                    let pls_items = asx::decode(content)?;
                    for item in pls_items {
                        set.insert(item.url);
                    }
                }
                None =>{
                    match content_small.find("[playlist]"){
                        Some(_) => {
                            let pls_items = pls::decode(content);
                            for item in pls_items {
                                set.insert(item.url);
                            }
                        }
                        None => {
                            let m3u_items = m3u::decode(content);
                            for item in m3u_items {
                                set.insert(item.url);
                            }
                        }
                    }
                }
            }
        }
    }
    let v: Vec<String> = set.into_iter().collect();
    Ok(v)
}

pub fn is_content_hls(content: &str) -> bool {
    if content.contains("EXT-X-STREAM-INF"){
        return true;
    }
    if content.contains("EXT-X-TARGETDURATION"){
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn xspf() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
<playlist version="1" xmlns="http://xspf.org/ns/0/">
    <trackList>
    <track>
        <title>Title</title>
        <identifier>Identifier</identifier>
        <location>http://this.is.an.example</location>
    </track>
    <track>
        <title>Title2</title>
        <identifier>Identifier2</identifier>
        <location>http://this.is.an.example2</location>
    </track>
    </trackList>
</playlist>"#;
        let items = crate::xspf::decode(s);
        assert!(items.is_ok());
        let items = items.unwrap();
        assert!(items.len() == 2);
        assert!(items[0].url == "http://this.is.an.example");
        assert!(items[0].title == "Title");
        assert!(items[0].identifier == "Identifier");
        assert!(items[1].url == "http://this.is.an.example2");
        assert!(items[1].title == "Title2");
        assert!(items[1].identifier == "Identifier2");
    }

    #[test]
    fn asx() {
        let s = r#"<asx version="3.0">
  <title>Test-Liste</title>
  <entry>
    <title>title1</title>
    <ref href="ref1"/>
  </entry>
  <entry>
    <title>title2</title>
    <ref href="ref2"/>
  </entry>
</asx>"#;
        let items = crate::asx::decode(s);
        assert!(items.is_ok());
        let items = items.unwrap();
        assert!(items.len() == 2);
        assert!(items[0].url == "ref1");
        assert!(items[0].title == "title1");
        assert!(items[1].url == "ref2");
        assert!(items[1].title == "title2");
    }

    #[test]
    fn m3u() {
        let items = crate::m3u::decode("http://this.is.an.example");
        assert!(items.len() == 1);
        assert!(items[0].url == "http://this.is.an.example");
    }

    #[test]
    fn pls() {
        let items = crate::pls::decode("[playlist]
File1=http://this.is.an.example
Title1=mytitle
        ");
        assert!(items.len() == 1);
        assert!(items[0].url == "http://this.is.an.example");
        assert!(items[0].title == "mytitle");
    }

    #[test]
    fn pls2() {
        let items = crate::pls::decode("[playlist]
File1=http://this.is.an.example
Title=mytitle
        ");
        assert!(items.len() == 1);
        assert!(items[0].url == "http://this.is.an.example");
        assert!(items[0].title == "mytitle");
    }

    #[test]
    fn pls3() {
        let items = crate::pls::decode("[Playlist]
File1=http://this.is.an.example
Title=mytitle
        ");
        assert!(items.len() == 1);
        assert!(items[0].url == "http://this.is.an.example");
        assert!(items[0].title == "mytitle");
    }
}
