use crate::window_search::WindowInfo;
use mouse_rs::types::keys::Keys;
use mouse_rs::Mouse;
use screenshots::image::RgbaImage;
use screenshots::Screen;

const GREEN_THRESHOLD: u8 = 200;
const OTHER_CHANNELS_THRESHOLD: u8 = 50;

pub struct Clicker {
    mouse: Mouse
}

impl Clicker {
    pub fn new() -> Clicker {
        let mouse = Mouse::new();
        Clicker { mouse }
    }

    pub fn create_screen(&self, target_window: &WindowInfo) -> RgbaImage {
        let screen = Screen::from_point(target_window.left, target_window.top).unwrap();

        let image = screen.capture_area(
            target_window.left,
            target_window.top,
            target_window.width() as u32,
            target_window.height() as u32
        ).unwrap();

        image
    }

    pub fn find_green_pixel(&self, image: &mut RgbaImage) -> Option<(i32, i32)> {
        for (x, y, pixel) in image.enumerate_pixels() {
            if pixel[0] == 205 &&
                pixel[1] == 220 &&
                pixel[2] == 0 {
                return Some((x as i32, y as i32));
            }
        }
        None
    }

    pub fn move_mouse_to(&self, x: i32, y: i32) {
        self.mouse.move_to(x,y).unwrap()
    }

    pub fn click(&self) {
        self.mouse.click(&Keys::LEFT).unwrap()
    }

    pub fn find_and_click_green(&self, target_window: &WindowInfo) {
        let screen = Screen::from_point(target_window.left, target_window.top).unwrap();
        let mut screenshot = self.create_screen(target_window);

        if let Some((rel_x, rel_y)) = self.find_green_pixel(&mut screenshot) {

            let abs_x = ((rel_x + target_window.left+10)) as f32 / screen.display_info.scale_factor;
            let abs_y = ((rel_y + target_window.top)+10) as f32 / screen.display_info.scale_factor;

            self.move_mouse_to(abs_x as i32, abs_y as i32);
            self.click();
        }
    }
}

