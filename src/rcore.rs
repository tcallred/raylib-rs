use crate::Color;
use crate::Shader;
use crate::Texture2D;
use crate::{Camera, Camera2D, Camera3D};
use crate::{Matrix, Ray};
use crate::{Vector2, Vector3};
use crate::{VrDeviceInfo, VrStereoConfig};
// Window-related functions
/// Initialize window and OpenGL context
pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe {
        raylib_ffi::InitWindow(width, height, raylib_ffi::rl_str!(title));
    }
}

/// Close window and unload OpenGL context
pub fn close_window() {
    unsafe {
        raylib_ffi::CloseWindow();
    }
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
pub fn window_should_close() -> bool {
    unsafe { raylib_ffi::WindowShouldClose() }
}

/// Check if window has been initialized successfully
pub fn is_window_ready() -> bool {
    unsafe { raylib_ffi::IsWindowReady() }
}

/// Check if window is currently fullscreen
pub fn is_window_fullscreen() -> bool {
    unsafe { raylib_ffi::IsWindowFullscreen() }
}

/// Check if window is currently hidden (only PLATFORM_DESKTOP)
pub fn is_window_hidden() -> bool {
    unsafe { raylib_ffi::IsWindowHidden() }
}

/// Check if window is currently minimized (only PLATFORM_DESKTOP)
pub fn is_window_minimized() -> bool {
    unsafe { raylib_ffi::IsWindowMinimized() }
}

/// Check if window is currently maximized (only PLATFORM_DESKTOP)
pub fn is_window_maximized() -> bool {
    unsafe { raylib_ffi::IsWindowMaximized() }
}

/// Check if window is currently focused (only PLATFORM_DESKTOP)
pub fn is_window_focused() -> bool {
    unsafe { raylib_ffi::IsWindowFocused() }
}

/// Check if window has been resized last frame
pub fn is_window_resized() -> bool {
    unsafe { raylib_ffi::IsWindowResized() }
}

/// Check if one specific window flag is enabled
pub fn is_window_state(flag: u32) -> bool {
    unsafe { raylib_ffi::IsWindowState(flag) }
}

/// Set window configuration state using flags (only PLATFORM_DESKTOP)
pub fn set_window_state(flags: u32) {
    unsafe { raylib_ffi::SetWindowState(flags) }
}

/// Clear window configuration state flags
pub fn clear_window_state(flags: u32) {
    unsafe { raylib_ffi::ClearWindowState(flags) }
}

/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
pub fn toggle_fullscreen() {
    unsafe { raylib_ffi::ToggleFullscreen() }
}

/// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
pub fn toggle_borderless_windowed() {
    unsafe { raylib_ffi::ToggleBorderlessWindowed() }
}

/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
pub fn maximize_window() {
    unsafe { raylib_ffi::MaximizeWindow() }
}

/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
pub fn minimize_window() {
    unsafe { raylib_ffi::MinimizeWindow() }
}

/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
pub fn restore_window() {
    unsafe { raylib_ffi::RestoreWindow() }
}

/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
pub fn set_window_icon(image: raylib_ffi::Image) {
    unsafe {
        raylib_ffi::SetWindowIcon(image);
    }
}

/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
pub fn set_window_icons(images: &mut [crate::Image]) {
    unsafe {
        raylib_ffi::SetWindowIcons(images.as_mut_ptr(), images.len() as i32);
    }
}

/// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
pub fn set_window_title(title: &str) {
    unsafe {
        raylib_ffi::SetWindowTitle(raylib_ffi::rl_str!(title));
    }
}

/// Set window position on screen (only PLATFORM_DESKTOP)
pub fn set_window_position(x: i32, y: i32) {
    unsafe {
        raylib_ffi::SetWindowPosition(x, y);
    }
}

/// Set monitor for the current window
pub fn set_window_monitor(monitor: i32) {
    unsafe {
        raylib_ffi::SetWindowMonitor(monitor);
    }
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_min_size(width: i32, height: i32) {
    unsafe {
        raylib_ffi::SetWindowMinSize(width, height);
    }
}

/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_max_size(width: i32, height: i32) {
    unsafe {
        raylib_ffi::SetWindowMaxSize(width, height);
    }
}

/// Set window dimensions
pub fn set_window_size(width: i32, height: i32) {
    unsafe {
        raylib_ffi::SetWindowSize(width, height);
    }
}

/// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
pub fn set_window_opacity(opacity: f32) {
    unsafe {
        raylib_ffi::SetWindowOpacity(opacity);
    }
}

/// Set window focused (only PLATFORM_DESKTOP)
pub fn set_window_focused() {
    unsafe {
        raylib_ffi::SetWindowFocused();
    }
}

/// Get native window handle
pub fn get_window_handle() -> *mut std::ffi::c_void {
    unsafe { raylib_ffi::GetWindowHandle() }
}

/// Get current screen width
pub fn get_screen_width() -> i32 {
    unsafe { raylib_ffi::GetScreenWidth() }
}

/// Get current screen height
pub fn get_screen_height() -> i32 {
    unsafe { raylib_ffi::GetScreenHeight() }
}

/// Get current render width (it considers HiDPI)
pub fn get_render_width() -> i32 {
    unsafe { raylib_ffi::GetRenderWidth() }
}

/// Get current render height (it considers HiDPI)
pub fn get_render_height() -> i32 {
    unsafe { raylib_ffi::GetRenderHeight() }
}

/// Get number of connected monitors
pub fn get_monitor_count() -> i32 {
    unsafe { raylib_ffi::GetMonitorCount() }
}

/// Get current connected monitor
pub fn get_current_monitor() -> i32 {
    unsafe { raylib_ffi::GetCurrentMonitor() }
}

/// Get specified monitor position
pub fn get_monitor_position(monitor: i32) -> Vector2 {
    unsafe { raylib_ffi::GetMonitorPosition(monitor) }
}

/// Get specified monitor width (current video mode used by monitor)
pub fn get_monitor_width(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorWidth(monitor) }
}

/// Get specified monitor height (current video mode used by monitor)
pub fn get_monitor_height(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorHeight(monitor) }
}

/// Get specified monitor physical width in millimetres
pub fn get_monitor_physical_width(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorPhysicalWidth(monitor) }
}

/// Get specified monitor physical height in millimetres
pub fn get_monitor_physical_height(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorPhysicalHeight(monitor) }
}

/// Get specified monitor refresh rate
pub fn get_monitor_refresh_rate(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorRefreshRate(monitor) }
}

/// Get window position XY on monitor
pub fn get_window_position() -> Vector2 {
    unsafe { raylib_ffi::GetWindowPosition() }
}

/// Get window scale DPI factor
pub fn get_window_scale_dpi() -> Vector2 {
    unsafe { raylib_ffi::GetWindowScaleDPI() }
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub fn get_monitor_name(monitor: i32) -> &'static str {
    unsafe {
        std::ffi::CStr::from_ptr(raylib_ffi::GetMonitorName(monitor))
            .to_str()
            .unwrap()
    }
}

/// Set clipboard text content
pub fn set_clipboard_text(text: &str) {
    unsafe { raylib_ffi::SetClipboardText(raylib_ffi::rl_str!(text)) }
}

/// Get clipboard text content
pub fn get_clipboard_text() -> String {
    unsafe {
        let ptr = raylib_ffi::GetClipboardText();
        std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
pub fn enable_event_waiting() {
    unsafe { raylib_ffi::EnableEventWaiting() }
}

/// Disable waiting for events on EndDrawing(), automatic events polling
pub fn disable_event_waiting() {
    unsafe { raylib_ffi::DisableEventWaiting() }
}

// Cursor-related functions
/// Shows cursor
pub fn show_cursor() {
    unsafe { raylib_ffi::ShowCursor() }
}

/// Hides cursor
pub fn hide_cursor() {
    unsafe { raylib_ffi::HideCursor() }
}

/// Check if cursor is not visible
pub fn is_cursor_hidden() -> bool {
    unsafe { raylib_ffi::IsCursorHidden() }
}

/// Enables cursor (unlock cursor)
pub fn enable_cursor() {
    unsafe { raylib_ffi::EnableCursor() }
}

/// Disables cursor (lock cursor)
pub fn disable_cursor() {
    unsafe { raylib_ffi::DisableCursor() }
}

/// Check if cursor is on the screen
pub fn is_cursor_on_screen() -> bool {
    unsafe { raylib_ffi::IsCursorOnScreen() }
}

/// Drawing-related functions

/// Set background color (framebuffer clear color)
pub fn clear_background(color: Color) {
    unsafe {
        raylib_ffi::ClearBackground(color);
    }
}

/// Setup canvas (framebuffer) to start drawing
pub fn begin_drawing() {
    unsafe {
        raylib_ffi::BeginDrawing();
    }
}

/// End canvas drawing and swap buffers (double buffering)
pub fn end_drawing() {
    unsafe {
        raylib_ffi::EndDrawing();
    }
}

/// Begin 2D mode with custom camera (2D)
pub fn begin_mode_2d(camera: Camera2D) {
    unsafe {
        raylib_ffi::BeginMode2D(camera);
    }
}

/// Ends 2D mode with custom camera
pub fn end_mode_2d() {
    unsafe {
        raylib_ffi::EndMode2D();
    }
}

/// Begin 3D mode with custom camera (3D)
pub fn begin_mode_3d(camera: Camera3D) {
    unsafe {
        raylib_ffi::BeginMode3D(camera);
    }
}

/// Ends 3D mode and returns to default 2D orthographic mode
pub fn end_mode_3d() {
    unsafe {
        raylib_ffi::EndMode3D();
    }
}

/// Begin drawing to render texture
pub fn begin_texture_mode(target: crate::RenderTexture2D) {
    unsafe {
        raylib_ffi::BeginTextureMode(target);
    }
}

/// Ends drawing to render texture
pub fn end_texture_mode() {
    unsafe {
        raylib_ffi::EndTextureMode();
    }
}

/// Begin custom shader drawing
pub fn begin_shader_mode(shader: crate::Shader) {
    unsafe {
        raylib_ffi::BeginShaderMode(shader);
    }
}

/// End custom shader drawing (use default shader)
pub fn end_shader_mode() {
    unsafe {
        raylib_ffi::EndShaderMode();
    }
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
pub fn begin_blend_mode(mode: i32) {
    unsafe {
        raylib_ffi::BeginBlendMode(mode);
    }
}

/// End blending mode (reset to default: alpha blending)
pub fn end_blend_mode() {
    unsafe {
        raylib_ffi::EndBlendMode();
    }
}

/// Begin scissor mode (define screen area for following drawing)
pub fn begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        raylib_ffi::BeginScissorMode(x, y, width, height);
    }
}

/// End scissor mode
pub fn end_scissor_mode() {
    unsafe {
        raylib_ffi::EndScissorMode();
    }
}

/// Begin stereo rendering (requires VR simulator)
pub fn begin_vr_stereo_mode(config: crate::VrStereoConfig) {
    unsafe {
        raylib_ffi::BeginVrStereoMode(config);
    }
}

/// End stereo rendering (requires VR simulator)
pub fn end_vr_stereo_mode() {
    unsafe {
        raylib_ffi::EndVrStereoMode();
    }
}

// VR stereo config functions for VR simulator
/// Load VR stereo config for VR simulator device parameters
pub fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
    unsafe { raylib_ffi::LoadVrStereoConfig(device) }
}

/// Unload VR stereo config
pub fn unload_vr_stereo_config(config: VrStereoConfig) {
    unsafe { raylib_ffi::UnloadVrStereoConfig(config) }
}

// Shader management functions
/// Load shader from files and bind default locations
pub fn load_shader(vs_file_name: &str, fs_file_name: &str) -> Shader {
    unsafe {
        raylib_ffi::LoadShader(
            raylib_ffi::rl_str!(vs_file_name),
            raylib_ffi::rl_str!(fs_file_name),
        )
    }
}

/// Load shader from code strings and bind default locations
pub fn load_shader_from_memory(vs_code: &str, fs_code: &str) -> Shader {
    unsafe {
        raylib_ffi::LoadShaderFromMemory(raylib_ffi::rl_str!(vs_code), raylib_ffi::rl_str!(fs_code))
    }
}

/// Check if a shader is ready
pub fn is_shader_ready(shader: Shader) -> bool {
    unsafe { raylib_ffi::IsShaderReady(shader) }
}

/// Get shader uniform location
pub fn get_shader_location(shader: Shader, uniform_name: &str) -> i32 {
    unsafe { raylib_ffi::GetShaderLocation(shader, raylib_ffi::rl_str!(uniform_name)) }
}

/// Get shader attribute location
pub fn get_shader_location_attrib(shader: Shader, attrib_name: &str) -> i32 {
    unsafe { raylib_ffi::GetShaderLocationAttrib(shader, raylib_ffi::rl_str!(attrib_name)) }
}

/// Set shader uniform value
pub fn set_shader_value(
    shader: Shader,
    loc_index: i32,
    value: *const std::ffi::c_void,
    uniform_type: i32,
) {
    unsafe { raylib_ffi::SetShaderValue(shader, loc_index, value, uniform_type) }
}

/// Set shader uniform value vector
pub fn set_shader_value_v(
    shader: Shader,
    loc_index: i32,
    value: *const std::ffi::c_void,
    uniform_type: i32,
    count: i32,
) {
    unsafe { raylib_ffi::SetShaderValueV(shader, loc_index, value, uniform_type, count) }
}

/// Set shader uniform value (matrix 4x4)
pub fn set_shader_value_matrix(shader: Shader, loc_index: i32, mat: Matrix) {
    unsafe { raylib_ffi::SetShaderValueMatrix(shader, loc_index, mat) }
}

/// Set shader uniform value for texture (sampler2d)
pub fn set_shader_value_texture(shader: Shader, loc_index: i32, texture: Texture2D) {
    unsafe { raylib_ffi::SetShaderValueTexture(shader, loc_index, texture) }
}

/// Unload shader from GPU memory (VRAM)
pub fn unload_shader(shader: Shader) {
    unsafe { raylib_ffi::UnloadShader(shader) }
}

// Screen-space-related functions
/// Get a ray trace from mouse position
pub fn get_mouse_ray(mouse_position: Vector2, camera: Camera) -> Ray {
    unsafe { raylib_ffi::GetMouseRay(mouse_position, camera) }
}

/// Get camera transform matrix (view matrix)
pub fn get_camera_matrix(camera: Camera) -> Matrix {
    unsafe { raylib_ffi::GetCameraMatrix(camera) }
}

/// Get camera 2d transform matrix
pub fn get_camera_matrix_2d(camera: Camera2D) -> Matrix {
    unsafe { raylib_ffi::GetCameraMatrix2D(camera) }
}

/// Get the screen space position for a 3d world space position
pub fn get_world_to_screen(position: Vector3, camera: Camera) -> Vector2 {
    unsafe { raylib_ffi::GetWorldToScreen(position, camera) }
}

/// Get the world space position for a 2d camera screen space position
pub fn get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { raylib_ffi::GetScreenToWorld2D(position, camera) }
}

/// Get size position for a 3d world space position
pub fn get_world_to_screen_ex(
    position: Vector3,
    camera: Camera,
    width: i32,
    height: i32,
) -> Vector2 {
    unsafe { raylib_ffi::GetWorldToScreenEx(position, camera, width, height) }
}

/// Get the screen space position for a 2d camera world space position
pub fn get_world_to_screen_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { raylib_ffi::GetWorldToScreen2D(position, camera) }
}

// Timing-related functions
/// Set target FPS (maximum)
pub fn set_target_fps(fps: i32) {
    unsafe { raylib_ffi::SetTargetFPS(fps) }
}

/// Get time in seconds for last frame drawn (delta time)
pub fn get_frame_time() -> f32 {
    unsafe { raylib_ffi::GetFrameTime() }
}

/// Get elapsed time in seconds since InitWindow()
pub fn get_time() -> f64 {
    unsafe { raylib_ffi::GetTime() }
}

/// Get current FPS
pub fn get_fps() -> i32 {
    unsafe { raylib_ffi::GetFPS() }
}

// Random values generation functions

/// Set the seed for the random number generator.
pub fn set_random_seed(seed: u32) {
    unsafe {
        raylib_ffi::SetRandomSeed(seed);
    }
}

/// Get a random value between min and max (both included).
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { raylib_ffi::GetRandomValue(min, max) }
}

/// Load random values sequence, no values repeated.
pub fn load_random_sequence(count: u32, min: i32, max: i32) -> Vec<i32> {
    unsafe {
        let sequence_ptr = raylib_ffi::LoadRandomSequence(count, min, max);
        let sequence = std::slice::from_raw_parts(sequence_ptr, count as usize).to_vec();
        raylib_ffi::UnloadRandomSequence(sequence_ptr);
        sequence
    }
}

// Misc. functions

/// Takes a screenshot of the current screen (filename extension defines format).
pub fn take_screenshot(file_name: &str) {
    unsafe {
        raylib_ffi::TakeScreenshot(raylib_ffi::rl_str!(file_name));
    }
}

/// Setup init configuration flags (view FLAGS).
pub fn set_config_flags(flags: u32) {
    unsafe {
        raylib_ffi::SetConfigFlags(flags);
    }
}

/// Open URL with the default system browser (if available).
pub fn open_url(url: &str) {
    unsafe {
        raylib_ffi::OpenURL(raylib_ffi::rl_str!(url));
    }
}

// NOTE: Following functions implemented in module [utils]
//------------------------------------------------------------------

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...).
pub fn trace_log(log_level: i32, text: &str) {
    unsafe {
        raylib_ffi::TraceLog(log_level, raylib_ffi::rl_str!(text));
    }
}

/// Set the current threshold (minimum) log level.
pub fn set_trace_log_level(log_level: i32) {
    unsafe {
        raylib_ffi::SetTraceLogLevel(log_level);
    }
}

/// Internal memory allocator.
pub fn mem_alloc(size: u32) -> *mut std::ffi::c_void {
    unsafe { raylib_ffi::MemAlloc(size) }
}

/// Internal memory reallocator.
pub fn mem_realloc(ptr: *mut std::ffi::c_void, size: u32) -> *mut std::ffi::c_void {
    unsafe { raylib_ffi::MemRealloc(ptr, size) }
}

/// Internal memory free.
pub fn mem_free(ptr: *mut std::ffi::c_void) {
    unsafe { raylib_ffi::MemFree(ptr) }
}

/// Set custom trace log.
pub fn set_trace_log_callback(callback: crate::TraceLogCallback) {
    unsafe {
        raylib_ffi::SetTraceLogCallback(callback);
    }
}

// Files management functions (continued)

/// Load file data as a byte array (read).
pub fn load_file_data(file_name: &str) -> Option<Vec<u8>> {
    unsafe {
        let mut data_size = 0;
        let data_ptr = raylib_ffi::LoadFileData(raylib_ffi::rl_str!(file_name), &mut data_size);
        if data_ptr.is_null() {
            None
        } else {
            let data = std::slice::from_raw_parts(data_ptr, data_size as usize).to_vec();
            raylib_ffi::UnloadFileData(data_ptr);
            Some(data)
        }
    }
}

/// Save data to file from byte array (write), returns true on success.
pub fn save_file_data(file_name: &str, data: &[u8]) -> bool {
    unsafe {
        let result = raylib_ffi::SaveFileData(
            raylib_ffi::rl_str!(file_name),
            data.as_ptr() as *mut std::ffi::c_void,
            data.len() as i32,
        );
        result
    }
}

// Compression/Encoding functionality (continued)

/// Compress data (DEFLATE algorithm), memory must be freed.
pub fn compress_data(data: &[u8]) -> Option<(Vec<u8>, i32)> {
    unsafe {
        let mut comp_data_size = 0;
        let comp_data_ptr =
            raylib_ffi::CompressData(data.as_ptr(), data.len() as i32, &mut comp_data_size);
        if comp_data_ptr.is_null() {
            None
        } else {
            let comp_data =
                std::slice::from_raw_parts(comp_data_ptr, comp_data_size as usize).to_vec();
            raylib_ffi::MemFree(comp_data_ptr as *mut std::ffi::c_void);
            Some((comp_data, comp_data_size))
        }
    }
}

/// Decompress data (DEFLATE algorithm), memory must be freed.
pub fn decompress_data(comp_data: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let mut data_size = 0;
        let data_ptr =
            raylib_ffi::DecompressData(comp_data.as_ptr(), comp_data.len() as i32, &mut data_size);
        if data_ptr.is_null() {
            None
        } else {
            let data = std::slice::from_raw_parts(data_ptr, data_size as usize).to_vec();
            raylib_ffi::MemFree(data_ptr as *mut std::ffi::c_void);
            Some(data)
        }
    }
}

/// Unload automation events list from file.
pub fn unload_automation_event_list(mut list: crate::AutomationEventList) {
    unsafe {
        raylib_ffi::UnloadAutomationEventList(&mut list);
    }
}

/// Export automation events list as a text file.
pub fn export_automation_event_list(list: crate::AutomationEventList, file_name: &str) -> bool {
    unsafe { raylib_ffi::ExportAutomationEventList(list, raylib_ffi::rl_str!(file_name)) }
}

/// Set automation event list to record to.
pub fn set_automation_event_list(list: &mut crate::AutomationEventList) {
    unsafe {
        raylib_ffi::SetAutomationEventList(list);
    }
}

/// Set automation event internal base frame to start recording.
pub fn set_automation_event_base_frame(frame: i32) {
    unsafe {
        raylib_ffi::SetAutomationEventBaseFrame(frame);
    }
}

/// Start recording automation events (AutomationEventList must be set).
pub fn start_automation_event_recording() {
    unsafe {
        raylib_ffi::StartAutomationEventRecording();
    }
}

/// Stop recording automation events.
pub fn stop_automation_event_recording() {
    unsafe {
        raylib_ffi::StopAutomationEventRecording();
    }
}

/// Play a recorded automation event.
pub fn play_automation_event(event: crate::AutomationEvent) {
    unsafe {
        raylib_ffi::PlayAutomationEvent(event);
    }
}

/// Load automation events list from file, None for an empty list.
pub fn load_automation_event_list(file_name: &str) -> crate::AutomationEventList {
    unsafe { raylib_ffi::LoadAutomationEventList(raylib_ffi::rl_str!(file_name)) }
}

// Input-related functions: keyboard
/// Check if a key has been pressed once
pub fn is_key_pressed(key: crate::enums::KeyboardKey) -> bool {
    unsafe { raylib_ffi::IsKeyPressed(key as i32) }
}

// Camera System Functions
/// Update camera position for selected mode
pub fn update_camera(camera: &mut crate::Camera, mode: crate::enums::CameraMode) {
    unsafe { raylib_ffi::UpdateCamera(camera, mode as i32) }
}
