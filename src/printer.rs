use crate::linter::LintFailed;
use anyhow::{Context, Error};
use colored::*;
use std::cmp;
use std::fs::File;
use std::io::{Read, Write};
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

enum TermCapture<C, N> {
    Capturable(C),
    Noncapturable(N),
}

// -------------------------------------------------------------------------------------------------
// Printer
// -------------------------------------------------------------------------------------------------

static CHAR_CR: u8 = 0x0d;
static CHAR_LF: u8 = 0x0a;

pub struct Printer {
    term: TermCapture<Vec<u8>, Option<Box<StdoutTerminal>>>,
}

impl Printer {
    #[cfg_attr(tarpaulin, skip)]
    pub fn new(capturable: bool) -> Printer {
        Printer {
            term: if capturable {
                TermCapture::Capturable(Vec::new())
            } else {
                TermCapture::Noncapturable(term::stdout())
            },
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    fn write(&mut self, dat: &str, color: Color) {
        match self.term {
            TermCapture::Noncapturable(Some(ref mut term)) => {
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
            }
            TermCapture::Noncapturable(None) => {
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
            TermCapture::Capturable(ref mut buf) => {
                // NOTE: Capturable terminal is only for unit testability,
                // but no checks are made on color.
                let _ = write!(buf, "{}", dat);
            }
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    fn with_pos<F: FnMut(usize, usize, usize, usize, Option<usize>)>(
        src: &str,
        print_pos: usize,
        mut func: F,
    ) {
        let mut pos = 0;
        let mut column = 1;
        let mut last_lf = None;
        while pos < src.len() {
            if src.as_bytes()[pos] == CHAR_LF {
                column += 1;
                last_lf = Some(pos);
            }

            if print_pos == pos {
                let row = if let Some(last_lf) = last_lf {
                    pos - last_lf
                } else {
                    pos + 1
                };
                let mut next_crlf = pos;
                while next_crlf < src.len() {
                    if src.as_bytes()[next_crlf] == CHAR_CR || src.as_bytes()[next_crlf] == CHAR_LF
                    {
                        break;
                    }
                    next_crlf += 1;
                }

                func(pos, column, row, next_crlf, last_lf);
            }

            pos += 1;
        }
    }

    fn get_pos(src: &str, print_pos: usize) -> Option<(usize, usize)> {
        let mut pos = 0;
        let mut column = 1;
        let mut last_lf = None;
        while pos < src.len() {
            if src.as_bytes()[pos] == CHAR_LF {
                column += 1;
                last_lf = Some(pos);
            }

            if print_pos == pos {
                let row = if let Some(last_lf) = last_lf {
                    pos - last_lf
                } else {
                    pos + 1
                };
                let mut next_crlf = pos;
                while next_crlf < src.len() {
                    if src.as_bytes()[next_crlf] == CHAR_CR || src.as_bytes()[next_crlf] == CHAR_LF
                    {
                        break;
                    }
                    next_crlf += 1;
                }

                return Some((row, column));
            }

            pos += 1;
        }
        None
    }

    #[cfg_attr(tarpaulin, skip)]
    fn print_single(
        &mut self,
        src: &str,
        print_pos: usize,
        header: &str,
        path: &Path,
        hint: Option<&str>,
    ) {
        Printer::with_pos(src, print_pos, |pos, column, row, next_crlf, _last_lf| {
            self.write(header, Color::BrightRed);
            self.write(
                &format!("\t{}:{}:{}", path.to_string_lossy(), column, row),
                Color::BrightBlue,
            );
            self.write(
                &format!(
                    "\t{}",
                    String::from_utf8_lossy(&src.as_bytes()[pos..next_crlf])
                ),
                Color::White,
            );
            if let Some(hint) = hint {
                self.write(&format!("\thint: {}", hint), Color::BrightYellow);
            }
            self.write("\n", Color::Reset);
        });
    }

    #[cfg_attr(tarpaulin, skip)]
    fn print_pretty(
        &mut self,
        src: &str,
        print_pos: usize,
        print_len: usize,
        header: &str,
        description: &str,
        path: &Path,
        hint: Option<&str>,
        reason: Option<&str>,
    ) {
        Printer::with_pos(src, print_pos, |pos, column, row, next_crlf, last_lf| {
            self.write(header, Color::BrightRed);

            let beg = if let Some(last_lf) = last_lf {
                if next_crlf > last_lf {
                    last_lf + 1
                } else {
                    next_crlf
                }
            } else {
                0
            };

            let column = if beg == 0 && next_crlf == 0 {
                // Position is first line and first line is empty
                0
            } else {
                column
            };

            let column_len = format!("{}", column).len();

            self.write(&format!(": {}\n", description), Color::BrightWhite);

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

            self.write(
                &format!(
                    " {}\n",
                    String::from_utf8_lossy(&src.as_bytes()[beg..next_crlf])
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
                    "^".repeat(cmp::min(print_pos + print_len, next_crlf) - print_pos)
                ),
                Color::BrightYellow,
            );

            if let Some(hint) = hint {
                self.write(&format!(" hint  : {}\n", hint), Color::BrightYellow);
            }

            if let Some(reason) = reason {
                self.write(
                    &format!("{}|", " ".repeat(column_len + 1)),
                    Color::BrightBlue,
                );

                self.write(
                    &format!(
                        " {}{}",
                        " ".repeat(pos - beg),
                        " ".repeat(cmp::min(print_pos + print_len, next_crlf) - print_pos)
                    ),
                    Color::Yellow,
                );

                self.write(&format!(" reason: {}\n", reason), Color::Yellow);
            }

            self.write("\n", Color::Reset);
        });
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_failed(
        &mut self,
        failed: &LintFailed,
        single: bool,
        github_actions: bool,
    ) -> Result<(), Error> {
        let mut f = File::open(&failed.path)
            .with_context(|| format!("failed to open: '{}'", failed.path.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        if single {
            self.print_single(&s, failed.beg, "Fail", &failed.path, Some(&failed.hint));
        } else {
            self.print_pretty(
                &s,
                failed.beg,
                failed.len,
                "Fail",
                &failed.name,
                &failed.path,
                Some(&failed.hint),
                Some(&failed.reason),
            );
        }

        if github_actions {
            if let Some((row, column)) = Printer::get_pos(&s, failed.beg) {
                println!(
                    "::error file={},line={},col={}::{}",
                    failed.path.to_string_lossy(),
                    column,
                    row,
                    failed.hint
                );
            }
        }

        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_parse_error(
        &mut self,
        path: &Path,
        error_pos: usize,
        single: bool,
    ) -> Result<(), Error> {
        let mut f = File::open(path)
            .with_context(|| format!("failed to open: '{}'", path.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        if single {
            self.print_single(&s, error_pos, "Error", path, Some("parse error"));
        } else {
            self.print_pretty(&s, error_pos, 1, "Error", "parse error", path, None, None);
        }
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_preprocess_error(
        &mut self,
        path: &Path,
        error_pos: usize,
        single: bool,
    ) -> Result<(), Error> {
        let mut f = File::open(path)
            .with_context(|| format!("failed to open: '{}'", path.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        if single {
            self.print_single(&s, error_pos, "Error", path, Some("preprocess error"));
        } else {
            self.print_pretty(&s, error_pos, 1, "Error", "preprocess error", path, None, None);
        }
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_error(&mut self, msg: &str) -> Result<(), Error> {
        self.write("Error", Color::BrightRed);
        self.write(&format!(": {}", msg), Color::BrightWhite);
        self.write("\n", Color::Reset);
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_warning(&mut self, msg: &str) -> Result<(), Error> {
        self.write("Warning", Color::BrightYellow);
        self.write(&format!(": {}", msg), Color::BrightWhite);
        self.write("\n", Color::Reset);
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_info(&mut self, msg: &str) -> Result<(), Error> {
        self.write("Info", Color::BrightGreen);
        self.write(&format!(": {}", msg), Color::BrightWhite);
        self.write("\n", Color::Reset);
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print_error_type(&mut self, error: Error) -> Result<(), Error> {
        let mut cause = error.chain();
        self.write("Error", Color::BrightRed);
        self.write(&format!(": {}", cause.next().unwrap()), Color::BrightWhite);
        self.write("\n", Color::Reset);
        for x in cause {
            self.write(&format!("  caused by: {}\n", x), Color::Reset);
        }
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn print(&mut self, msg: &str) -> Result<(), Error> {
        self.write(msg, Color::Reset);
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn println(&mut self, msg: &str) -> Result<(), Error> {
        self.write(msg, Color::Reset);
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
