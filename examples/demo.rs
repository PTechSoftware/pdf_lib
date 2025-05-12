use pdf_lib::document::PdfDocument;
use pdf_lib::table::PdfTable;

fn main() -> std::io::Result<()> {
    let mut doc = PdfDocument::new("Demo", (595.0, 842.0));

    let page = doc.add_page();
    page.draw_text("Factura #123", 50.0, 780.0, 14.0, "Helvetica");
    page.draw_image("logo.png", 450.0, 770.0, 100.0, 50.0);

    let mut table = PdfTable::new(vec!["Producto", "Cant.", "Precio", "Total"]);
    table.add_row(vec!["Mate", "2", "$100", "$200"]);
    table.add_row(vec!["Termo", "1", "$500", "$500"]);
    page.draw_table(table, 50.0, 700.0)?;

    doc.save("factura.pdf")
}
