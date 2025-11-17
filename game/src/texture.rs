use gl::{self, TextureTarget};

use image::RgbaImage;

pub struct Texture {
    texture: gl::Texture,
}

pub struct TextureCreateInfo {
    pub rgba_image: RgbaImage,
    pub internal_format: gl::BaseInternalFormat,
    pub mip_level: i32,
    pub wrap_s: gl::TextureWrapMode,
    pub wrap_t: gl::TextureWrapMode,
    pub min_filter: gl::InterpolationMode,
    pub mag_filter: gl::InterpolationMode,
    pub mipmap_interpolation: Option<gl::InterpolationMode>,
}

impl From<TextureCreateInfo> for Texture {
    fn from(info: TextureCreateInfo) -> Self {
        let TextureCreateInfo {
            rgba_image,
            internal_format,
            mip_level,
            wrap_s,
            wrap_t,
            min_filter,
            mag_filter,
            mipmap_interpolation,
        } = info;

        let mut texture = gl::Texture::create1(TextureTarget::Texture2D);
        texture.parameter_i_wrap_s(wrap_s);
        texture.parameter_i_wrap_t(wrap_t);
        texture.parameter_i_mag_filter(mag_filter);
        texture.parameter_i_min_filter(min_filter, mipmap_interpolation);

        texture.storage_2d(
            1,
            internal_format,
            rgba_image.width() as i32,
            rgba_image.height() as i32,
        );

        texture.sub_image_2d(
            mip_level,
            (0, 0),
            (rgba_image.width() as i32, rgba_image.height() as i32),
            gl::PixelDataFormat::RGBA,
            gl::PixelDataType::UnsignedByte,
            rgba_image.as_ptr(),
        );

        texture.generate_mipmap();

        Texture { texture }
    }
}

impl Texture {
    pub fn bind_to_unit(&self, unit: u32) {
        gl::active_texture_gl_texture(unit);
        self.texture.bind(gl::TextureTarget::Texture2D);
    }

    pub fn id(&self) -> u32 {
        self.texture.id()
    }
}
