//! Simple boilerplate for SDL init.
use crate::prelude::Unit;
use color_eyre::{Report, Result};
use rental::rental;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

rental! {
    /// `rental` required wrapper.
    pub mod creator_censor {
        use super::*;

        /// Black magic to hide away the [`TextureCreator`] with the [`Texture`] it creates.
        #[rental]
        pub struct CreatorCensor {
            /// [`TextureCreator`] that bounds lifetime of textures.
            creator: Box<TextureCreator<WindowContext>>,
            /// [`Texture`] for drawing to the window.
            texture: Texture<'creator>,
        }
    }
}
pub use creator_censor::CreatorCensor;

impl CreatorCensor {
    /// Creates both a [`TextureCreator`] and [`Texture`] with given size.
    pub fn with_size(canvas: &Canvas<Window>, width: usize, height: usize) -> Result<Self> {
        let creator = canvas.texture_creator();
        Ok(
            CreatorCensor::try_new(Box::new(creator), |tc: &TextureCreator<_>| {
                create_texture(&tc, width, height)
            })
            .map_err(|e| e.0)?,
        )
    }

    /// Forwards to [`Texture::update`].
    pub fn update<R: Into<Option<Rect>>>(
        &mut self,
        rect: R,
        pixel_data: &[u8],
        pitch: usize,
    ) -> Unit {
        self.rent_mut(|texture| Ok(texture.update(rect, pixel_data, pitch)?))
    }

    /// Forwards to [`Texture::with_lock`].
    pub fn with_lock<F: FnOnce(&mut [u8], usize) -> R, R, R2: Into<Option<Rect>>>(
        &mut self,
        rect: R2,
        func: F,
    ) -> Result<R> {
        self.rent_mut(|texture| texture.with_lock(rect, func).map_err(Report::msg))
    }
}

/// Convenience struct holding all the details of a singleton texture for the entire window.
pub struct UnitaryWindow {
    /// Screen width.
    pub width: usize,
    /// Screen height.
    pub height: usize,
    /// SDL [`Canvas`].
    pub canvas: Canvas<Window>,
    /// [`Texture`], along with its creator.
    pub texture: CreatorCensor,
}

impl UnitaryWindow {
    /// Initializes video subsystem, creates window with given size and singular whole-window texture.
    pub fn init(width: usize, height: usize) -> Result<Self> {
        let canvas = sdl_init_window(width, height)?;
        let texture = CreatorCensor::with_size(&canvas, width, height)?;
        Ok(Self {
            width: width,
            height: height,
            canvas: canvas,
            texture: texture,
        })
    }

    /// Forwards to [`Texture::update`].
    pub fn update<R: Into<Option<Rect>>>(
        &mut self,
        rect: R,
        pixel_data: &[u8],
        pitch: usize,
    ) -> Unit {
        self.texture.update(rect, pixel_data, pitch)
    }

    /// Forwards to [`Texture::with_lock`].
    pub fn with_lock<F: FnOnce(&mut [u8], usize) -> R, R, R2: Into<Option<Rect>>>(
        &mut self,
        rect: R2,
        func: F,
    ) -> Result<R> {
        self.texture.with_lock(rect, func)
    }

    /// Redraws the entire window.
    pub fn redraw(&mut self) -> Unit {
        self.texture
            .rent(|texture| self.canvas.copy(&texture, None, None).map_err(Report::msg))?;
        self.canvas.present();
        Ok(())
    }
}

/// Boilerplate to init video subsystem only and obtain a [`Canvas`].
///
/// Call canvas.texture_creator() next.
pub fn sdl_init_window(width: usize, height: usize) -> Result<Canvas<Window>> {
    let sdl_context = sdl2::init().map_err(Report::msg)?;
    let video_subsystem = sdl_context.video().map_err(Report::msg)?;
    let window = video_subsystem
        .window("Display", width as u32, height as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(Report::msg)?;
    window.into_canvas().build().map_err(Report::msg)
}

/// Create a texture bound to the lifetime of this [`TextureCreator`].
pub fn create_texture<'a>(
    creator: &'a TextureCreator<WindowContext>,
    width: usize,
    height: usize,
) -> Result<Texture<'a>> {
    Ok(creator.create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)?)
}
