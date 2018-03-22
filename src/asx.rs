use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Clone)]
pub struct PlaylistItem {
    pub title: String,
    pub url: String,
}

pub fn decode_playlist(content: &str) -> Vec<PlaylistItem> {
    let mut list = vec![];
    let mut item = PlaylistItem {
        title: String::from(""),
        url: String::from(""),
    };

    let mut reader = Reader::from_str(content);
    reader.trim_text(true);
    let mut xml_stack = vec![];
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                xml_stack.push(reader.decode(e.name()).to_string().to_lowercase());

                let path = xml_stack.join("/");
                for a in e.attributes() {
                    match a {
                        Ok(a) => {
                            let key = reader.decode(a.key).to_string().to_lowercase();
                            let value = reader.decode(&a.value).to_string();
                            if path == "asx/entry/ref" {
                                if key == "href" {
                                    item.url = value;
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }

                xml_stack.pop();
            }
            Ok(Event::Start(ref e)) => {
                xml_stack.push(reader.decode(e.name()).to_string().to_lowercase());

                let path = xml_stack.join("/");
                for a in e.attributes() {
                    match a {
                        Ok(a) => {
                            let key = reader.decode(a.key).to_string().to_lowercase();
                            let value = reader.decode(&a.value).to_string();
                            if path == "asx/entry/ref" {
                                if key == "href" {
                                    item.url = value;
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Ok(Event::End(_)) => {
                let path = xml_stack.join("/");
                if path == "asx/entry" {
                    list.push(item.clone());
                    item.title = String::from("");
                    item.url = String::from("");
                }
                xml_stack.pop();
            }
            Ok(Event::Text(e)) => {
                let path = xml_stack.join("/");
                if path == "asx/entry/title" {
                    item.title = e.unescape_and_decode(&reader).expect("msg").clone();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                println!("Error at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
    }

    list
}
