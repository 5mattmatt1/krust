// This is all for them sweet sweet monochrome, monospace fonts

fn dcnds_char(ichr: u8) -> [u8; 8]
{
    let mut ichr_set: [u8; 8] = [0; 8];
    // for i in (7, -1).step_by(-1) // range_step(7, -1, -1)
    // {
    let mut i: isize = 7;
    while i > -1
    {
        ichr_set[(7-i) as usize] = (ichr >> i) & 1;
        i -= 1;
    }
    return ichr_set;
}

/*
 * Currently only need a u8 for width and height.
 */

/* @brief Decodes a font from a raw u8 array
 * @param rfont_data Raw u8 array of palette indices
 * @param fb Memory address of the framebuffer
 * @param iw Image width in pixels
 * @param ih Image height in pixels
 * @param cw Character width in pixels
 * @param ch Character height in pixels
 * @param cwbs Character width bit shift (log base 2 of character width)
 */
// TODO: Make this generic
// In the meantime use fonts from baking pi that are 128x96
pub unsafe fn draw_font(rfont_data: [u8; 1536], fb: u32, iw: u8, ih: u8, cw: u8, ch: u8, cwbs: u8)
{
    // Get ahold of some sweet generic arrays
    let mut offset: usize = 0;
    let mut ichr_set: [u8; 8];
    let mut cI: usize = 0;
    
    // use core::iter::range_step;
    use uart::uart_putc;
    use postman::draw_pixel;
    // let mut y: usize = 0;
    for y in (0..ih).step_by(ch as usize) 
    {
        // uart_putc('y');
        // uart_putc((0x30 + y) as char);
        // uart_putc('\n');
        for x in (0..iw).step_by(cw as usize)
        {
            // uart_putc('x');
            // uart_putc((0x30 + x) as char);
            // uart_putc('\n');
            for row in 0..ch
            {
                offset = ((x as usize) << 1) + (((y as usize) * iw as usize) >> cwbs as usize) + row as usize;
                ichr_set = dcnds_char(rfont_data[offset]);
                for px in 0..8
                {
                    if ichr_set[px] == 1
                    {
                        /* White */
                        draw_pixel(fb, (x as usize + px as usize) as u32, 
                                        (y as usize + row as usize) as u32, 
                                        640, 3, 0xFFFFFF);
                    } else 
                    {
                        /* Black */
                        draw_pixel(fb, (x as usize + px as usize) as u32, 
                                        (y as usize + row as usize) as u32, 
                                        640, 3, 0x0);
                    }
                }
                cI += 8;
            }
        }
    }
}

/*
fn draw_font()
{

}
*/