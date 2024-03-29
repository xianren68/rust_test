use crossterm::terminal::ClearType;
use crossterm::{cursor, event::*};
use crossterm::{event, execute, queue, terminal};
use std::io::{self, stdout, Write};
use std::time::Duration;

/// 版本号
const VERSION: &str = "0.0.1";
/// 程序退出时的收尾工作
struct CleanUp;

/// Drop释放资源在panic后依然会执行
impl Drop for CleanUp {
    fn drop(&mut self) {
        // 恢复终端为规范模式
        terminal::disable_raw_mode().expect("cloud not turn off raw mode");
        // 程序退出时清屏
        OutPut::clear_screen().expect("Error")
    }
}

/// 读取输入
struct Reader;

impl Reader {
    fn read_key(&self) -> Result<KeyEvent, io::Error> {
        loop {
            // 设置超时
            if event::poll(Duration::from_millis(5000))? {
                // 只匹配按键输入
                if let Event::Key(e) = event::read()? {
                    return Ok(e);
                }
            } else {
                println!("not input yet\r")
            }
        }
    }
}

/// 编辑
struct Editor {
    reader: Reader,
    output: OutPut,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: OutPut::new(),
        }
    }
    // 判断终端是否能够继续输入
    fn process_keypress(&mut self) -> Result<bool, io::Error> {
        match self.reader.read_key()? {
            // ctrl + q 退出程序
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => Ok(false),
            // 控制光标位置
            KeyEvent {
                code:
                    direction @ (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::End
                    | KeyCode::Home),
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            } => {
                self.output.move_cursor(direction);
                return Ok(true);
            }
            // 控制翻页
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            } => {
                (0..self.output.win_size.1).for_each(|_|{
                    self.output.move_cursor(if matches!(val,KeyCode::PageUp){
                        KeyCode::Up
                    }else {
                        KeyCode::Down
                    })
                });
                return Ok(true)
                }
            _ => Ok(true),
        }
    }
    // 运行
    fn run(&mut self) -> Result<bool, io::Error> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}

/// 输出
struct OutPut {
    // 终端窗口的大小
    win_size: (usize, usize),
    // 编辑的内容
    editor_contents: EditorContents,
    // 光标位置
    cursor_controller: CursorController,
}

impl OutPut {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
        }
    }
    // 绘制编辑器左侧波浪线与一些版本信息
    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for i in 0..screen_rows {
            if i == screen_rows - 3 {
                let mut welcome = format!("Pound editor  -- Version{}", VERSION);
                if welcome.len() > screen_columns {
                    welcome.truncate(screen_columns);
                }
                // 版本信息居中显示
                let mut padding = (screen_columns - welcome.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1;
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome)
            } else {
                self.editor_contents.push('~')
            }
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            // 最后一行不用\r\n
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n")
            }
        }
    }
    // 清理终端
    fn clear_screen() -> Result<(), io::Error> {
        // 清除终端数据
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        // 移动光标到左上角
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
    // 刷新
    fn refresh_screen(&mut self) -> Result<(), io::Error> {
        queue!(
            self.editor_contents,
            cursor::Hide, // 隐藏光标
            cursor::MoveTo(0, 0)
        )?;
        let CursorController {
            cursor_x, cursor_y, ..
        } = self.cursor_controller;
        self.draw_rows();
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }
    fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction)
    }
}

/// 标准输出流缓冲区，方便一次性往终端写入数据
struct EditorContents {
    content: String,
}
impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    // 添加字节
    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }
    // 添加字符串
    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}
/// 实现输入trait
impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // 判断是否能够转换为utf8字符串
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        // 将字符串写入标准输出流
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

/// 光标控制
struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    screen_columns: usize,
    screen_rows: usize,
}

impl CursorController {
    fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
        }
    }
    fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            // 上下左右
            KeyCode::Up => {
                if self.cursor_y != 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y != self.screen_rows - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x != self.screen_columns - 1 {
                    self.cursor_x += 1;
                }
            }
            // 回到顶部
            KeyCode::Home => {
                self.cursor_y = 0;
            }
            // 到底部
            KeyCode::End => {
                self.cursor_y = self.screen_rows;
            }
            _ => unimplemented!(),
        }
    }
}
pub fn run() -> Result<(), io::Error> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
