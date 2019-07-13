/*
 * MIT License
 *
 * Copyright (c) 2018-2019 Matthew Henderson <mattw2018@hotmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
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

pub unsafe fn draw_string(rfont_data: [u8; 1536], 
                        fb: u32, fb_width: u32, s: &str, 
                        ox: u8, oy: u8, 
                        iw: u8, ih: u8, 
                        cw: u8, ch: u8, 
                        cwbs: u8)
{
    let mut oxi = ox;
    for byte in s.as_bytes()
    {
        draw_char(rfont_data, fb, fb_width, *byte as char, 
                    oxi, oy, iw, ih, cw, ch, cwbs);
        oxi += cw;
    }
}

pub unsafe fn draw_char(rfont_data: [u8; 1536], fb: u32, fb_width: u32, chr: char, ox: u8, oy: u8, iw: u8, ih: u8, cw: u8, ch: u8, cwbs: u8)
{
    let x = ((chr as u8 - 0x20) & 15) << 3;
    let y = ((chr as u8 - 0x20) >> 4) << 4;
    // Lots of hard coded in values atm
    // Will change as I create a font struct taht will hold some of this data

    let mut offset: usize = 0;
    let mut ichr_set: [u8; 8];
    use crate::postman::draw_pixel;
    for row in 0..ch
    {
        offset = ((x as usize) << 1) + (((y as usize) * iw as usize) >> cwbs as usize) + row as usize;
        ichr_set = dcnds_char(rfont_data[offset]);
        for px in 0..8
        {
            if ichr_set[px] == 1
            {
                /* Black */
                draw_pixel(fb, (ox as usize + px as usize) as u32, 
                                (oy as usize + row as usize) as u32, 
                                fb_width, 4, 0x0);
            } else 
            {
                /* Yellow */
                draw_pixel(fb, (ox as usize + px as usize) as u32, 
                                (oy as usize + row as usize) as u32, 
                                fb_width, 4, 0xFF44FFFF);
            }
        }
    }
}

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
pub unsafe fn draw_font(rfont_data: [u8; 1536], fb: u32, iw: u8, ih: u8, cw: u8, ch: u8, cwbs: u8, fb_width: u32)
{
    // Get ahold of some sweet generic arrays
    let mut offset: usize = 0;
    let mut ichr_set: [u8; 8];
    let mut cI: usize = 0;
    
    // Lots of hard coded in values atm
    // Will change as I create a font struct taht will hold some of this data

    use crate::postman::draw_pixel;
    for y in (0..ih).step_by(ch as usize) 
    {
        for x in (0..iw).step_by(cw as usize)
        {
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
                                        fb_width, 3, 0xFFFFFF);
                    } else 
                    {
                        /* Black */
                        draw_pixel(fb, (x as usize + px as usize) as u32, 
                                        (y as usize + row as usize) as u32, 
                                        fb_width, 3, 0x0);
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