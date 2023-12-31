use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const BOUNDARY: &str = "==========\r\n";
const OUTPUT_DIR: &str = "output";


fn get_sections(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap();
    let splited: Vec<String> = content.split(BOUNDARY).map(|s| s.to_owned()).collect();
    splited
}

fn get_clip(section: &str) -> Result<HashMap<&str, &str>, String> {
    let mut book: HashMap<&str, &str> = HashMap::new();
    let lines: Vec<&str> = section.split("\r\n").filter(|row| row != &"").collect();
    if lines.len() != 3 {
        return Err(format!("lines != 3 {:?}", lines));
    }
    book.insert("book", lines[0]);
    let re: Regex = Regex::new(r"(\d+)-\d+").unwrap();
    let caps = re.captures(lines[1]);
    match caps {
        Some(result) => {
            let position = result.get(0).unwrap().as_str();
            book.insert("position", position);
            book.insert("content", lines[2]);
            Ok(book)
        }
        None => Err(format!("Err on {:?}", lines)),
    }
}

fn export_txt(clips: &HashMap<&str, HashMap<&str, &str>>) {
        for (book_name, book_content) in clips.iter() {
        let lines: Vec<&str> = book_content.values().cloned().collect();
        let mut path: PathBuf = [OUTPUT_DIR, book_name].iter().collect();
        path.set_extension("md");
        fs::write(path, lines.join("\n\n---\n\n")).expect("Failed to write file");
    }
}

fn main() {
    let sections: Vec<String> = get_sections("My Clippings.txt");
    let mut clips: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    for section in &sections {
        let clip = get_clip(section);
        match clip {
            Ok(clip) => {
                if clips.contains_key(clip["book"]) {
                    clips
                        .get_mut(clip["book"])
                        .map(|val| val.entry(clip["position"]).or_insert(clip["content"]));
                } else {
                    let mut p = HashMap::new();
                    p.insert(clip["position"], clip["content"]);
                    clips.insert(clip["book"], p);
                }
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }
    export_txt(&clips);
}
