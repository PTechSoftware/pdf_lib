use crate::traits::pdf_represent::PdfRepresentatation;
use std::collections::HashMap;
#[derive(Debug,Default)]
pub struct PdfDictionary {
    map : HashMap<String, String>,
}

impl PdfDictionary {
    pub fn new() -> Self {
        Self{
            map: HashMap::new(),
        }
    }
    
    pub fn add_value(&mut self, key:&str, value:String) {
        self.map.insert(key.to_string(),value.clone());
    }
}

impl PdfRepresentatation for PdfDictionary {
    fn get_as_string(&self) -> (String, u64) {
        let mut str = String::with_capacity(self.map.len()*30);
        for (key, value) in &self.map {
            str.push_str(format!("/{} {}",key, value ).as_str());
        }
        let s = format!(
            "<<{}>>\n", str);
        let size = s.len() as u64;
        (s, size)
    }

    
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        let (body, _) = self.get_as_string();
        format!("{id} {generation} obj\n{body}\nendobj")
    }
}