use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::{self, terminal};
use std::time::Duration;

/// 用于将终端复原为规范模式
struct CleanUp;

/// Drop释放资源在panic后依然会执行
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("cloud not turn off raw mode")
    }
}

fn main() -> Result<(), std::io::Error> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(500))? {
            // 只匹配按键输入
            if let Event::Key(e) = event::read()? {
                match e {
                    // 设置退出为ctrl + q
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    } => return Ok(()),
                    _ => {}
                }
                println!("{:?}\r", e);
            }
        } else {
            println!("no input yet\r")
        }
    }
}
