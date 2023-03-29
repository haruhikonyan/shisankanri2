mod utils;

use wasm_bindgen::prelude::*;

use std::error::Error;
use std::fs;

// use headless_chrome::protocol::cdp::Page;
// use headless_chrome::Browser;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, scraping!");
}
#[wasm_bindgen]
pub fn square(num: i32) -> i32 {
    num * num
}

// #[wasm_bindgen]
// pub fn exec_screenshot() -> i32 {
//     let _ = screenshot("hoge");
//     12
// }
// // これ自体の動作確認は main.rs 作って cargo run すれば実行できる
// pub fn screenshot(_url: &str) -> Result<(), Box<dyn Error>> {
//     let browser = Browser::default()?;
//     let tab = browser.new_tab()?;

//     // tab.navigate_to(url)?;
//     tab.navigate_to("https://www.wikipedia.org")?;

//     tab.wait_until_navigated()?;

//     let png_data =
//         tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
//     fs::write("screenshot.png", &png_data)?;

//     Ok(())
// }
