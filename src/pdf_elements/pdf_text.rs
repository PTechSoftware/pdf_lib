
/*
BT
/F1 12 Tf        % Fuente F1 tamaño 12
100 750 Td       % Posición inicial: x=100, y=750
(Primera línea) Tj

0 -15 Td         % Baja 15 unidades en Y (nuevo renglón)
(Segunda línea) Tj

0 -15 Td         % Otro renglón hacia abajo
(Tercera línea) Tj
ET

Sistema de coordenadas
Origen (0,0) está en la esquina inferior izquierda por defecto.

Las unidades son puntos PDF (1 punto = 1/72 pulgadas).

Entonces si querés renglones tipo hoja A4 (altura 842 pt), tenés que empezar arriba e ir bajando en y.

*/
use crate::models::tm::Tm;

pub(crate)  struct PdfText {
    font_name: String,
    font_size: i32,
    ///Td => Relative movement joined to latest position
    td_position : (i32, i32),
    tm_position : Tm
}