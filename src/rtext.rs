use crate::{Color, Font, GlyphInfo, Image, Rectangle, Vector2};
// Text drawing functions
/// Get the default Font
pub fn get_font_default() -> Font {
    unsafe { raylib_ffi::GetFontDefault() }
}

/// Load font from file into GPU memory (VRAM)
pub fn load_font(file_name: &str) -> Font {
    unsafe { raylib_ffi::LoadFont(raylib_ffi::rl_str!(file_name)) }
}

/// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set
pub fn load_font_ex(
    file_name: &str,
    font_size: i32,
    codepoints: Option<&[i32]>,
    codepoint_count: i32,
) -> Font {
    let codepoints_ptr: *mut i32 = match codepoints {
        Some(cp) => {
            let mut cp_vec = cp.to_vec();
            let cp = &mut cp_vec[..];
            cp.as_mut_ptr()
        }
        None => std::ptr::null_mut(),
    };
    unsafe {
        raylib_ffi::LoadFontEx(
            raylib_ffi::rl_str!(file_name),
            font_size,
            codepoints_ptr,
            codepoint_count,
        )
    }
}

/// Load font from Image (XNA style)
pub fn load_font_from_image(image: Image, key: Color, first_char: i32) -> Font {
    unsafe { raylib_ffi::LoadFontFromImage(image, key, first_char) }
}

/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
pub fn load_font_from_memory(
    file_type: &str,
    file_data: &[u8],
    font_size: i32,
    codepoints: Option<&[i32]>,
    codepoint_count: i32,
) -> Font {
    let codepoints_ptr = match codepoints {
        Some(cp) => {
            let mut cp_vec = cp.to_vec();
            let cp = &mut cp_vec[..];
            cp.as_mut_ptr()
        }
        None => std::ptr::null_mut(),
    };
    unsafe {
        raylib_ffi::LoadFontFromMemory(
            raylib_ffi::rl_str!(file_type),
            file_data.as_ptr(),
            file_data.len() as i32,
            font_size,
            codepoints_ptr,
            codepoint_count,
        )
    }
}

/// Check if a font is ready
pub fn is_font_ready(font: Font) -> bool {
    unsafe { raylib_ffi::IsFontReady(font) }
}

/// Load font data for further use
pub fn load_font_data(
    file_data: &[u8],
    font_size: i32,
    codepoints: Option<&[i32]>,
    codepoint_count: i32,
    type_: i32,
) -> *mut GlyphInfo {
    let codepoints_ptr = match codepoints {
        Some(cp) => {
            let mut cp_vec = cp.to_vec();
            let cp = &mut cp_vec[..];
            cp.as_mut_ptr()
        }
        None => std::ptr::null_mut(),
    };
    unsafe {
        raylib_ffi::LoadFontData(
            file_data.as_ptr(),
            file_data.len() as i32,
            font_size,
            codepoints_ptr,
            codepoint_count,
            type_,
        )
    }
}

/// Generate image font atlas using chars info
pub fn gen_image_font_atlas(
    glyphs: &[GlyphInfo],
    glyph_recs: &mut Option<&mut [Rectangle]>,
    glyph_count: i32,
    font_size: i32,
    padding: i32,
    pack_method: i32,
) -> Image {
    let mut glyph_recs_ptr = match glyph_recs {
        Some(ref mut gr) => gr.as_mut_ptr(),
        None => std::ptr::null_mut(),
    };
    unsafe {
        raylib_ffi::GenImageFontAtlas(
            glyphs.as_ptr(),
            &mut glyph_recs_ptr,
            glyph_count,
            font_size,
            padding,
            pack_method,
        )
    }
}

/// Unload font chars info data (RAM)
pub fn unload_font_data(glyphs: *mut GlyphInfo, glyph_count: i32) {
    unsafe { raylib_ffi::UnloadFontData(glyphs, glyph_count) }
}

/// Unload font from GPU memory (VRAM)
pub fn unload_font(font: Font) {
    unsafe { raylib_ffi::UnloadFont(font) }
}

/// Export font as code file, returns true on success
pub fn export_font_as_code(font: Font, file_name: &str) -> bool {
    unsafe { raylib_ffi::ExportFontAsCode(font, raylib_ffi::rl_str!(file_name)) }
}

/// Draw current FPS
pub fn draw_fps(pos_x: i32, pos_y: i32) {
    unsafe {
        raylib_ffi::DrawFPS(pos_x, pos_y);
    }
}

/// Draw text (using default font)
pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawText(raylib_ffi::rl_str!(text), pos_x, pos_y, font_size, color);
    }
}

/// Draw text using font and additional parameters
pub fn draw_text_ex(
    font: Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    unsafe {
        raylib_ffi::DrawTextEx(
            font,
            raylib_ffi::rl_str!(text),
            position,
            font_size,
            spacing,
            tint,
        );
    }
}

/// Draw text using Font and pro parameters (rotation)
pub fn draw_text_pro(
    font: Font,
    text: &str,
    position: Vector2,
    origin: Vector2,
    rotation: f32,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    unsafe {
        raylib_ffi::DrawTextPro(
            font,
            raylib_ffi::rl_str!(text),
            position,
            origin,
            rotation,
            font_size,
            spacing,
            tint,
        );
    }
}

/// Draw one character (codepoint)
pub fn draw_text_codepoint(
    font: Font,
    codepoint: i32,
    position: Vector2,
    font_size: f32,
    tint: Color,
) {
    unsafe {
        raylib_ffi::DrawTextCodepoint(font, codepoint, position, font_size, tint);
    }
}

/// Draw multiple characters (codepoints)
pub fn draw_text_codepoints(
    font: Font,
    codepoints: &[i32],
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    unsafe {
        raylib_ffi::DrawTextCodepoints(
            font,
            codepoints.as_ptr(),
            codepoints.len() as i32,
            position,
            font_size,
            spacing,
            tint,
        );
    }
}

/// Set vertical line spacing when drawing with line-breaks
pub fn set_text_line_spacing(spacing: i32) {
    unsafe {
        raylib_ffi::SetTextLineSpacing(spacing);
    }
}

/// Measure string width for default font
pub fn measure_text(text: &str, font_size: i32) -> i32 {
    unsafe { raylib_ffi::MeasureText(raylib_ffi::rl_str!(text), font_size) }
}

/// Measure string size for Font
pub fn measure_text_ex(font: Font, text: &str, font_size: f32, spacing: f32) -> Vector2 {
    unsafe { raylib_ffi::MeasureTextEx(font, raylib_ffi::rl_str!(text), font_size, spacing) }
}

/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
pub fn get_glyph_index(font: Font, codepoint: i32) -> i32 {
    unsafe { raylib_ffi::GetGlyphIndex(font, codepoint) }
}

/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
pub fn get_glyph_info(font: Font, codepoint: i32) -> GlyphInfo {
    unsafe { raylib_ffi::GetGlyphInfo(font, codepoint) }
}

/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
pub fn get_glyph_atlas_rec(font: Font, codepoint: i32) -> Rectangle {
    unsafe { raylib_ffi::GetGlyphAtlasRec(font, codepoint) }
}

/// Load UTF-8 text encoded from codepoints array
pub fn load_utf8(codepoints: &[i32]) -> String {
    let length = codepoints.len() as i32;
    unsafe {
        let c_string = raylib_ffi::LoadUTF8(codepoints.as_ptr(), length);
        std::ffi::CStr::from_ptr(c_string).to_string_lossy().into_owned()
    }
}

/// Unload UTF-8 text encoded from codepoints array
pub fn unload_utf8(text: &str) {
    let c_string = std::ffi::CString::new(text).expect("Failed to convert to CString");
    unsafe {
        raylib_ffi::UnloadUTF8(c_string.into_raw());
    }
}

/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
pub fn load_codepoints(text: &str) -> Vec<i32> {
    let mut count = 0;
    let c_string = std::ffi::CString::new(text).expect("Failed to convert to CString");
    unsafe {
        let codepoints = raylib_ffi::LoadCodepoints(c_string.as_ptr(), &mut count);
        std::slice::from_raw_parts(codepoints, count as usize).to_vec()
    }
}

/// Unload codepoints data from memory
pub fn unload_codepoints(codepoints: Vec<i32>) {
    let ptr = codepoints.as_ptr() as *mut i32;
    std::mem::forget(codepoints); // Prevent Rust from deallocating the vector's memory
    unsafe {
        raylib_ffi::UnloadCodepoints(ptr);
    }
}

/// Get total number of codepoints in a UTF-8 encoded string
pub fn get_codepoint_count(text: &str) -> i32 {
    let c_string = std::ffi::CString::new(text).expect("Failed to convert to CString");
    unsafe {
        raylib_ffi::GetCodepointCount(c_string.as_ptr())
    }
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub fn get_codepoint(text: &str, codepoint_size: &mut i32) -> i32 {
    let c_string = std::ffi::CString::new(text).expect("Failed to convert to CString");
    unsafe {
        raylib_ffi::GetCodepoint(c_string.as_ptr(), codepoint_size)
    }
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub fn get_codepoint_next(text: &str, codepoint_size: &mut i32) -> i32 {
    let c_string = std::ffi::CString::new(text).expect("Failed to convert to CString");
    unsafe {
        raylib_ffi::GetCodepointNext(c_string.as_ptr(), codepoint_size)
    }
}

/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub fn get_codepoint_previous(text: &str, codepoint_size: &mut i32) -> i32 {
    let c_string = std::ffi::CString::new(text).expect("Failed to convert to CString");
    unsafe {
        raylib_ffi::GetCodepointPrevious(c_string.as_ptr(), codepoint_size)
    }
}

/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
pub fn codepoint_to_utf8(codepoint: i32, utf8_size: &mut i32) -> String {
    unsafe {
        let c_string = raylib_ffi::CodepointToUTF8(codepoint, utf8_size);
        std::ffi::CStr::from_ptr(c_string).to_string_lossy().into_owned()
    }
}
