use crate::traits::pdf_represent::PdfRepresentatation;
use std::collections::HashMap;

pub struct PdfDictionary {
    map : HashMap<String, String>,
}


impl PdfRepresentatation for PdfDictionary {
    fn get_as_string(&self) -> (String, u64) {
        let mut str = String::with_capacity(self.map.len()*30);
        for (key, value) in &self.map {
            str.push_str(format!("/{} /{} \n",key, value ).as_str());
        }
        let s = format!(
            "<<{}>>", str);
        let size = s.len() as u64;
        (s, size)
    }

    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        format!("obj {} {}\n {} \nendobject", id, generation, self.get_as_string().0)
    }
}