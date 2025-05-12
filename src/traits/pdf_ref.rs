pub trait PdfRef{
    /// Should return something like: [ 4 0 R ]
    fn get_reference(self) -> String;
}