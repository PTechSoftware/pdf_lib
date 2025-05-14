#[derive(Debug)]
pub struct PdfPageHandle {
    pub stream_id: u64,
    pub page_id: u64,
    pub content: String,
    pub xobjects: Vec<String>,
}

impl PdfPageHandle {

    pub fn add_image(&mut self, name: &str) {
        self.xobjects.push(name.to_string());
    }

    pub fn xobjects_entry(&self) -> Option<String> {
        if self.xobjects.is_empty() {
            None
        } else {
            let mut entry = String::new();
            for name in &self.xobjects {
                entry.push_str(&format!("/{name} {name} "));
            }
            Some(entry.trim_end().to_string())
        }
    }

    /// Agrega una línea de texto con fuente, tamaño y coordenadas absolutas
    pub fn add_text(&mut self, font: &str, size: i32, x: i32, y: i32, text: &str) {
        use std::fmt::Write;
        writeln!(
            &mut self.content,
            "BT /{} {} Tf {} {} Td ({}) Tj ET",
            font, size, x, y, text
        ).unwrap();
    }

    /// Agrega múltiples líneas de texto con interlineado controlado
    pub fn add_multiline_text(&mut self, font: &str, size: i32, start_x: i32, start_y: i32, line_spacing: i32, lines: &[&str]) {
        use std::fmt::Write;
        writeln!(
            &mut self.content,
            "BT /{} {} Tf {} {} Td",
            font, size, start_x, start_y
        ).unwrap();

        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                writeln!(&mut self.content, "0 -{} Td", line_spacing).unwrap();
            }
            writeln!(&mut self.content, "({}) Tj", line).unwrap();
        }

        writeln!(&mut self.content, "ET").unwrap();
    }

    /// Inserta un contenido PDF literal (útil para contenido más bajo nivel)
    pub fn add_raw(&mut self, raw: &str) {
        self.content.push_str(raw);
        self.content.push('\n');
    }

    /// Devuelve el largo del stream (sin necesidad de calcularlo aparte)
    pub fn stream_len(&self) -> usize {
        self.content.len()
    }

    /// Permite acceso mut al contenido si se quiere escribir manualmente
    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }
}
