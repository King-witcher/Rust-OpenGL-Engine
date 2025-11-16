use gl;

use image::RgbaImage;

pub struct Texture {
    width: i32,
    height: i32,
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

        const TARGET: gl::TextureTarget = gl::TextureTarget::Texture2D;

        let texture = gl::Texture::gen1();
        texture.bind(TARGET);
        gl::tex_parameter_i_wrap_s(TARGET, wrap_s);
        gl::tex_parameter_i_wrap_t(TARGET, wrap_t);
        gl::tex_parameter_i_mag_filter(TARGET, mag_filter);
        gl::tex_parameter_i_min_filter(TARGET, min_filter, mipmap_interpolation);

        unsafe {
            gl::tex_image_2d(
                TARGET,
                mip_level,
                internal_format,
                rgba_image.width() as i32,
                rgba_image.height() as i32,
                gl::PixelDataFormat::RGBA,
                gl::PixelDataType::UnsignedByte,
                rgba_image.as_ptr(),
            )
        };

        gl::generate_mipmap(TARGET);

        Texture {
            width: rgba_image.width() as _,
            height: rgba_image.height() as _,
            texture,
        }
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
