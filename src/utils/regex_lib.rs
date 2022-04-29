//--- IMPORTS ---//
use std::collections::HashMap;
use regex::Regex;
//--- END IMPORTS ---//

pub struct LibRegex {
    pub regex_map: HashMap<String, String>,
}

impl LibRegex {
    pub fn new() -> Self {
        LibRegex {
            regex_map: HashMap::new(),
        }
    }

    pub fn build_regex_map(&mut self) -> &mut Self {
        self.regex_map = HashMap::from([
            (r"^\d{4}-\d{2}-\d{2}$".to_string(), "Cool, I'll be there".to_string()),
            (r"test".to_string(), "Oh, so you think you are a coder...".to_string())
        ]);
        self
    }

    pub fn regex_search(&mut self, query: &str) -> &str {
        for (search, _response) in &self.regex_map {
            let re = Regex::new(&search.as_str()).unwrap();
            if re.is_match(query) {
                match self.regex_map.get(search.as_str()) {
                    Some(resp) => return resp,
                    None => println!("None")
                }
            }
        }
        return "";
    }
}