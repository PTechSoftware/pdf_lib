


#[allow(dead_code)]
pub fn draw_table(rows: &[Vec<&str>], start_x: i32, start_y: i32, col_width: i32, row_height: i32) -> String {
    let mut out = String::new();
    let total_width = col_width * rows[0].len() as i32;
    let total_height = row_height * rows.len() as i32;

    // Bordes exteriores
    out += &format!("{} {} {} {} re S\n", start_x, start_y - total_height, total_width, total_height);

    // Líneas horizontales
    for i in 0..=rows.len() {
        let y = start_y - (i as i32 * row_height);
        out += &format!("{} {} m {} {} l S\n", start_x, y, start_x + total_width, y);
    }

    // Líneas verticales
    for i in 0..=rows[0].len() {
        let x = start_x + (i as i32 * col_width);
        out += &format!("{} {} m {} {} l S\n", x, start_y, x, start_y - total_height);
    }

    // Texto
    for (i, row) in rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let x = start_x + 10 + (j as i32 * col_width);
            let y = start_y - 15 - (i as i32 * row_height);
            out += &format!("BT /F1 10 Tf {} {} Td ({}) Tj ET\n", x, y, cell);
        }
    }

    out
}
