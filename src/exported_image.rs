use crate::mailbox::vc::Framebuffer;

pub struct ExportedImage<'a>
{
    pub width: u32,
    pub height: u32,
    pub palette: &'a [u32],
    pub array: &'a [u8],
}

impl<'a> ExportedImage<'a>
{
    pub fn render(&'a self, fb: &Framebuffer, x: u32, y: u32)
    {
        let mut color;
        let mut index: usize;
        for j in 0..self.height
        {
            for i in 0..self.width
            {
                index = ((j * self.width) + i) as usize;
                color = self.palette[self.array[index] as usize];
                fb.draw_pixel(i + x, j + y, color);
            }
        }
    }
}