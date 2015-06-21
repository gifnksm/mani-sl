#![feature(libc)]
#![feature(step_by)]
#![feature(str_char)]
#![feature(non_ascii_idents)]

extern crate libc;
extern crate ncurses;
extern crate unicode_width;

use std::thread;
use locale::Category;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

mod locale;

fn main() {
    locale::setlocale(Category::All, "");
    let scr = ncurses::initscr();

    unsafe {
        libc::funcs::posix01::signal::signal(libc::SIGINT, libc::SIG_IGN);
    }

    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::nodelay(scr, true);
    ncurses::leaveok(scr, true);
    ncurses::scrollok(scr, false);

    for x in (ncurses::COLS - 1 ..).step_by(-1) {
        ncurses::clear();
        match render_般若心経(100, 100, x, true) {
            Err(()) => panic!(),
            Ok(false) => break,
            Ok(true) => {}
        }
        ncurses::getch();
        ncurses::refresh();
        thread::sleep_ms(4);
    }

    ncurses::mvcur(0, ncurses::COLS - 1, ncurses::LINES - 1, 0);
    ncurses::endwin();
}

fn mvaddstr(y: i32, x: i32, s: &str) -> Result<(), ()> {
    if ncurses::mvaddstr(y, x, s) == ncurses::ERR {
        Err(())
    } else {
        Ok(())
    }
}

fn trim_right_by_width(mut s: &str, width: usize, is_cjk: bool) -> (&str, usize) {
    let mut w = 0;
    while w < width {
        if s.is_empty() { return (s, w) }
        let range = s.char_range_at_reverse(s.len());
        w += if is_cjk {
            range.ch.width_cjk().unwrap()
        } else {
            UnicodeWidthChar::width(range.ch).unwrap()
        };
        s = &s[.. range.next];
    }
    assert!(w >= width);
    (s, w)
}

fn trim_left_by_width(mut s: &str, width: usize, is_cjk: bool) -> (&str, usize) {
    if s.is_empty() { return (s, 0) }

    let mut w = 0;
    while w < width {
        if s.is_empty() { return (s, w) }
        let range = s.char_range_at(0);
        w += if is_cjk {
            range.ch.width_cjk().unwrap()
        } else {
            UnicodeWidthChar::width(range.ch).unwrap()
        };
        s = &s[range.next ..];
    }
    assert!(w >= width);
    (s, w)
}

fn slice_by_width(s: &str, width: usize, is_cjk: bool) -> (&str, &str, usize) {
    let mut taken_width = 0;
    let mut i = 0;
    loop {
        let range = s.char_range_at(i);

        let w = if is_cjk {
            range.ch.width_cjk().unwrap()
        } else {
            UnicodeWidthChar::width(range.ch).unwrap()
        };
        if w + taken_width > width {
            break
        }

        taken_width += w;
        i = range.next;
        if i >= s.len() { break }
    }

    assert!(taken_width <= width);
    (&s[.. i], &s[i ..], taken_width)
}

fn split_by_width(mut s: &str, width: usize, is_cjk: bool) -> Vec<&str> {
    let mut result = vec![];

    while !s.is_empty() {
        let (line, rest, _) = slice_by_width(s, width, is_cjk);
        result.push(line);
        s = rest;
    }
    result
}

fn render_line(y: i32, x: i32, mut line: &str, is_cjk: bool) -> Result<bool, ()> {
    if line.is_empty() { return Ok(false) }
    if x >= ncurses::COLS { return Ok(true) }
    if y >= ncurses::LINES || y < 0 { return Ok(false) }

    let render_width = (ncurses::COLS - x) as usize;

    let w = if is_cjk {
        line.width_cjk()
    } else {
        UnicodeWidthStr::width(line)
    };
    // let w = line.width(is_cjk);
    if w > render_width {
        line = trim_right_by_width(line, w - render_width, is_cjk).0;
        if line.is_empty() { return Ok(true) }
    }

    if x < 0 {
        let (line, w) = trim_left_by_width(line, (-x) as usize, is_cjk);
        if line.is_empty() { return Ok(false) }

        try!(mvaddstr(y, (w as i32) + x, line));
    } else {
        try!(mvaddstr(y, x, line));
    }

    Ok(true)
}

fn render_般若心経(repeat: usize, width: usize, x0: i32, is_cjk: bool) -> Result<bool, ()> {
    let s = 般若心経.replace("\n", "");
    let lines = split_by_width(&s, width, is_cjk);

    let num_line_blocks = (ncurses::LINES as usize) / (lines.len() + 1);
    let (d, m) = (repeat / num_line_blocks, repeat % num_line_blocks);

    let y0 = (ncurses::LINES - ((num_line_blocks * (lines.len() + 1)) as i32)) / 2;
    let mut cont = false;
    for i in 0 .. num_line_blocks {
        let mut num_column_blocks = d;
        if i < m { num_column_blocks += 1; }
        for j in 0 .. num_column_blocks {
            let x = x0 + ((width + 1) * j) as i32;
            let y = y0 + (i * (lines.len() + 1)) as i32;
            for (dy, line) in lines.iter().enumerate() {
                cont |= try!(render_line(y + (dy as i32), x, &line, is_cjk));
            }
        }
    }
    Ok(cont)
}

const 般若心経: &'static str = "
摩訶般若波羅蜜多心経
観自在菩薩行深般若波羅蜜多時照見五
蘊皆空度一切苦厄舎利子色不異空空不
異色色即是空空即是色受想行識亦復如
是舎利子是諸法空相不生不滅不垢不浄
不増不減是故空中無色無受想行識無眼
耳鼻舌身意無色声香味触法無眼界乃至
無意識界無無明亦無無明尽乃至無老死
亦無老死尽無苦集滅道無智亦無得以無
所得故菩提薩埵依般若波羅蜜多故心無
罣礙無罣礙故無有恐怖遠離一切顛倒夢
想究竟涅槃三世諸仏依般若波羅蜜多故
得阿耨多羅三藐三菩提故知般若波羅蜜
多是大神咒是大明咒是無上咒是無等等
咒能除一切苦真実不虚故説般若波羅蜜
多咒即説咒曰
掲諦掲諦波羅掲諦波羅僧掲諦菩提薩婆訶
般若心経";
