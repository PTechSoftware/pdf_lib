


pub struct PdfPage {
    /// Sets the reference to the parent object.
    parent : (i32, i32), // Ex. /Parent 2 0 R
    ///The page size in inches. n/72 to get the size
    media_box : (i32,i32, i32, i32),// Ex. /MediaBox [ 0 0 612 792 ]
    ///Te page cut . Similar to media Box
    crop_box : (i32, i32, i32, i32), //Ex. /CropBox [ 0 0 500 600 ]
    /// Allows to configure the rotation of the page
    rotate : i32 ,// Ex. /Rotate 90
    /// Handle the sclae on differents resx
    user_unit : f32, //Ex. /UserUnit 3.14159
}