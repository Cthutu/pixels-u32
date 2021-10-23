use pixels::Pixels;

/// Extends the Pixel structure interface in the Pixels crate.
pub trait PixelsExt {
    /// Provides a 32-bit slice rather than the 8-bit one using `get_frame`.
    fn get_frame_u32(&mut self) -> &mut [u32];
}

/// Implements the extra methods for Pixels.
impl PixelsExt for Pixels {
    fn get_frame_u32(&mut self) -> &mut [u32] {
        if let Ok(slice) = bytemuck::try_cast_slice_mut::<u8, u32>(self.get_frame()) {
            slice
        } else {
            panic!("Pixels are not aligned on a 4-byte boundary");
        }
    }
}
