use crate::linter::LintFailed;
use colored::*;
use failure::{Error, ResultExt};
use std::cmp;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use term::{self, color, StdoutTerminal};

// -------------------------------------------------------------------------------------------------
// Color
// -------------------------------------------------------------------------------------------------

#[derive(PartialEq)]
#[allow(dead_code)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
}

// -------------------------------------------------------------------------------------------------
// Printer
// -------------------------------------------------------------------------------------------------

static CHAR_CR: u8 = 0x0d;
static CHAR_LF: u8 = 0x0a;

pub struct Printer {
    term: Option<Box<StdoutTerminal>>,
}

impl Printer {
    #[cfg_attr(tarpaulin, skip)]
    pub fn new() -> Printer {
        Printer {
            term: term::stdout(),
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print(&mut self, failed: &LintFailed, simple: bool) -> Result<(), Error> {
        if simple {
            self.print_simple(failed)?;
        } else {
            self.print_pretty(failed)?;
        }

        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn write(&mut self, dat: &str, color: Color) {
        if let Some(ref mut term) = self.term {
            let term_color = match color {
                Color::Black => color::BLACK,
                Color::Red => color::RED,
                Color::Green => color::GREEN,
                Color::Yellow => color::YELLOW,
                Color::Blue => color::BLUE,
                Color::Magenta => color::MAGENTA,
                Color::Cyan => color::CYAN,
                Color::White => color::WHITE,
                Color::BrightBlack => color::BRIGHT_BLACK,
                Color::BrightRed => color::BRIGHT_RED,
                Color::BrightGreen => color::BRIGHT_GREEN,
                Color::BrightYellow => color::BRIGHT_YELLOW,
                Color::BrightBlue => color::BRIGHT_BLUE,
                Color::BrightMagenta => color::BRIGHT_MAGENTA,
                Color::BrightCyan => color::BRIGHT_CYAN,
                Color::BrightWhite => color::BRIGHT_WHITE,
                Color::Reset => color::BLACK,
            };
            if color == Color::Reset {
                let _ = term.reset();
            } else {
                let _ = term.fg(term_color);
            }
            let _ = write!(term, "{}", dat);
        } else {
            let colored = match color {
                Color::Black => dat.black(),
                Color::Red => dat.red(),
                Color::Green => dat.green(),
                Color::Yellow => dat.yellow(),
                Color::Blue => dat.blue(),
                Color::Magenta => dat.magenta(),
                Color::Cyan => dat.cyan(),
                Color::White => dat.white(),
                Color::BrightBlack => dat.bright_black(),
                Color::BrightRed => dat.bright_red(),
                Color::BrightGreen => dat.bright_green(),
                Color::BrightYellow => dat.bright_yellow(),
                Color::BrightBlue => dat.bright_blue(),
                Color::BrightMagenta => dat.bright_magenta(),
                Color::BrightCyan => dat.bright_cyan(),
                Color::BrightWhite => dat.bright_white(),
                Color::Reset => dat.clear(),
            };
            print!("{}", colored);
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    fn print_simple(&mut self, failed: &LintFailed) -> Result<(), Error> {
        let mut f = File::open(&failed.path)
            .with_context(|_| format!("failed to open: '{}'", failed.path.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        let mut pos = 0;
        let mut column = 1;
        let mut last_lf = 0;
        while pos < s.len() {
            if s.as_bytes()[pos] == CHAR_LF {
                column += 1;
                last_lf = pos;
            }

            if failed.beg == pos {
                let row = pos - last_lf;
                let mut next_crlf = pos;
                while next_crlf < s.len() {
                    if s.as_bytes()[next_crlf] == CHAR_CR || s.as_bytes()[next_crlf] == CHAR_LF {
                        break;
                    }
                    next_crlf += 1;
                }

                self.write("Fail", Color::BrightRed);
                self.write(
                    &format!("\t{}:{}:{}", failed.path.to_string_lossy(), column, row),
                    Color::BrightBlue,
                );
                self.write(
                    &format!(
                        "\t{}",
                        String::from_utf8_lossy(&s.as_bytes()[pos..next_crlf])
                    ),
                    Color::White,
                );
                self.write(&format!("\thint: {}", failed.hint), Color::BrightYellow);
                self.write("\n", Color::Reset);
            }

            pos += 1;
        }
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn print_pretty(&mut self, failed: &LintFailed) -> Result<(), Error> {
        let mut f = File::open(&failed.path)
            .with_context(|_| format!("failed to open: '{}'", failed.path.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        let mut pos = 0;
        let mut column = 1;
        let mut last_lf = None;
        while pos < s.len() {
            if s.as_bytes()[pos] == CHAR_LF {
                column += 1;
                last_lf = Some(pos);
            }

            if failed.beg == pos {
                let row = if let Some(last_lf) = last_lf {
                    pos - last_lf
                } else {
                    pos + 1
                };
                let mut next_crlf = pos;
                while next_crlf < s.len() {
                    if s.as_bytes()[next_crlf] == CHAR_CR || s.as_bytes()[next_crlf] == CHAR_LF {
                        break;
                    }
                    next_crlf += 1;
                }

                self.write("Fail", Color::BrightRed);

                let column_len = format!("{}", column).len();

                self.write(&format!(": {}\n", failed.name), Color::BrightWhite);

                self.write("   -->", Color::BrightBlue);

                self.write(
                    &format!(" {}:{}:{}\n", failed.path.to_string_lossy(), column, row),
                    Color::White,
                );

                self.write(
                    &format!("{}|\n", " ".repeat(column_len + 1)),
                    Color::BrightBlue,
                );

                self.write(&format!("{} |", column), Color::BrightBlue);

                let beg = if let Some(last_lf) = last_lf {
                    last_lf + 1
                } else {
                    0
                };

                self.write(
                    &format!(
                        " {}\n",
                        String::from_utf8_lossy(&s.as_bytes()[beg..next_crlf])
                    ),
                    Color::White,
                );

                self.write(
                    &format!("{}|", " ".repeat(column_len + 1)),
                    Color::BrightBlue,
                );

                self.write(
                    &format!(
                        " {}{}",
                        " ".repeat(pos - beg),
                        "^".repeat(cmp::min(failed.beg + failed.len, next_crlf) - failed.beg)
                    ),
                    Color::BrightYellow,
                );

                self.write(&format!(" hint  : {}\n", failed.hint), Color::BrightYellow);

                self.write(
                    &format!("{}|", " ".repeat(column_len + 1)),
                    Color::BrightBlue,
                );

                self.write(
                    &format!(
                        " {}{}",
                        " ".repeat(pos - beg),
                        " ".repeat(cmp::min(failed.beg + failed.len, next_crlf) - failed.beg)
                    ),
                    Color::Yellow,
                );

                self.write(&format!(" reason: {}\n", failed.reason), Color::Yellow);

                self.write("\n", Color::Reset);
            }

            pos += 1;
        }
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_parse_error(&mut self, path: &Path, error_pos: usize) -> Result<(), Error> {
        let mut f = File::open(path)
            .with_context(|_| format!("failed to open: '{}'", path.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        let mut pos = 0;
        let mut column = 1;
        let mut last_lf = None;
        while pos < s.len() {
            if s.as_bytes()[pos] == CHAR_LF {
                column += 1;
                last_lf = Some(pos);
            }

            if error_pos == pos {
                let row = if let Some(last_lf) = last_lf {
                    pos - last_lf
                } else {
                    pos + 1
                };
                let mut next_crlf = pos;
                while next_crlf < s.len() {
                    if s.as_bytes()[next_crlf] == CHAR_CR || s.as_bytes()[next_crlf] == CHAR_LF {
                        break;
                    }
                    next_crlf += 1;
                }

                self.write("Error", Color::BrightRed);

                let column_len = format!("{}", column).len();

                self.write(&format!(": parse error\n"), Color::BrightWhite);

                self.write("   -->", Color::BrightBlue);

                self.write(
                    &format!(" {}:{}:{}\n", path.to_string_lossy(), column, row),
                    Color::White,
                );

                self.write(
                    &format!("{}|\n", " ".repeat(column_len + 1)),
                    Color::BrightBlue,
                );

                self.write(&format!("{} |", column), Color::BrightBlue);

                let beg = if let Some(last_lf) = last_lf {
                    last_lf + 1
                } else {
                    0
                };

                self.write(
                    &format!(
                        " {}\n",
                        String::from_utf8_lossy(&s.as_bytes()[beg..next_crlf])
                    ),
                    Color::White,
                );

                self.write(
                    &format!("{}|", " ".repeat(column_len + 1)),
                    Color::BrightBlue,
                );

                self.write(
                    &format!(
                        " {}{}",
                        " ".repeat(pos - beg),
                        "^".repeat(cmp::min(error_pos + 1, next_crlf) - error_pos)
                    ),
                    Color::BrightYellow,
                );

                self.write("\n\n", Color::Reset);
            }

            pos += 1;
        }
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_error(&mut self, error: &str) -> Result<(), Error> {
        self.write("Error", Color::BrightRed);
        self.write(&format!(": {}", error), Color::BrightWhite);
        self.write("\n", Color::Reset);
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_info(&mut self, error: &str) -> Result<(), Error> {
        self.write("Info", Color::BrightGreen);
        self.write(&format!(": {}", error), Color::BrightWhite);
        self.write("\n", Color::Reset);
        Ok(())
    }

    //#[cfg_attr(tarpaulin, skip)]
    //fn print_summary(
    //    &mut self,
    //    path_checked: &[(PathBuf, Vec<Checked>)],
    //    _verbose: bool,
    //    start_time: SystemTime,
    //) -> Result<(), Error> {
    //    self.write(
    //        "- Summary ----------------------------------------------------------------------\n\n",
    //        Color::BrightGreen,
    //    );

    //    let cnt_file = path_checked.len();
    //    let cnt_checked = path_checked.iter().fold(0, |sum, (_, y)| sum + y.len());
    //    let cnt_pass = path_checked.iter().fold(0, |sum, (_, y)| {
    //        sum + y.iter().filter(|x| x.state == CheckedState::Pass).count()
    //    });
    //    let cnt_fail = path_checked.iter().fold(0, |sum, (_, y)| {
    //        sum + y.iter().filter(|x| x.state == CheckedState::Fail).count()
    //    });
    //    let cnt_skip = path_checked.iter().fold(0, |sum, (_, y)| {
    //        sum + y.iter().filter(|x| x.state == CheckedState::Skip).count()
    //    });

    //    self.write(&format!("  * Checked files : {}\n", cnt_file), Color::Reset);
    //    self.write(
    //        &format!(
    //            "  * Checked points: {} ( Pass: {}, Fail: {}, Skip: {} )\n",
    //            cnt_checked, cnt_pass, cnt_fail, cnt_skip
    //        ),
    //        Color::Reset,
    //    );

    //    let elapsed = start_time.elapsed()?;
    //    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_micros() as f64 * 1e-6;
    //    self.write(&format!("  * Elapsed time  : {}s\n", elapsed), Color::Reset);
    //    self.write("\n", Color::Reset);

    //    Ok(())
    //}
}
