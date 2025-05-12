pub struct PdfTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl PdfTable {
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers.into_iter().map(String::from).collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        self.rows.push(row.into_iter().map(String::from).collect());
    }
}
