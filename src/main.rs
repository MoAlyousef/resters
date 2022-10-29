use fltk::{enums::*, prelude::*, *};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};
use json_tools::{Buffer, BufferType, Lexer, Span, TokenType};
use ureq::Error;

fn main() {
    let a = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply();
    let mut buf = text::TextBuffer::default();
    let mut sbuf = text::TextBuffer::default();
    let styles: Vec<text::StyleTableEntry> = vec![
        text::StyleTableEntry {
            color: Color::Red,
            font: Font::Courier,
            size: 14,
        },
        text::StyleTableEntry {
            color: Color::Blue,
            font: Font::Courier,
            size: 14,
        },
        text::StyleTableEntry {
            color: Color::Black,
            font: Font::Courier,
            size: 14,
        },
        text::StyleTableEntry {
            color: Color::Green.darker(),
            font: Font::Courier,
            size: 14,
        },
    ];
    let mut w = window::Window::default()
        .with_size(600, 400)
        .with_label("Resters");
    w.set_xclass("resters");
    let mut col = group::Flex::default()
        .with_type(group::FlexType::Column)
        .with_size(590, 390)
        .center_of_parent();
    let mut row = group::Flex::default();
    col.set_size(&row, 30);
    let mut choice = menu::Choice::default();
    choice.add_choice("GET|POST");
    choice.set_value(0);
    row.set_size(&choice, 80);
    let f = frame::Frame::default().with_label("https://");
    row.set_size(&f, 60);
    let mut inp = input::Input::default();
    inp.set_trigger(CallbackTrigger::EnterKeyAlways);
    let mut info = button::Button::default().with_label("  ℹ️");
    info.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    info.set_label_size(18);
    info.set_callback(move |_| {
        dialog::message_default("Resters was created using Rust and fltk-rs. It is MIT licensed!")
    });
    row.set_size(&info, 30);
    row.end();
    let mut disp = text::TextDisplay::default();
    disp.wrap_mode(text::WrapMode::AtBounds, 0);
    disp.set_buffer(buf.clone());
    disp.set_highlight_data(sbuf.clone(), styles);
    let mut row = group::Flex::default();
    col.set_size(&row, 20);
    let f = frame::Frame::default().with_label("Status: ");
    row.set_size(&f, 80);
    let mut status = frame::Frame::default().with_align(Align::Left | Align::Inside);
    row.end();
    col.end();
    w.end();
    w.make_resizable(true);
    w.show();

    inp.set_callback(move |inp| {
        status.set_label("");
        buf.set_text("");
        sbuf.set_text("");
        let mut path = inp.value();
        if !path.starts_with("https://") {
            path = String::from("https://") + &path;
        }
        let req = match choice.value() {
            0 => ureq::get(&path),
            1 => ureq::post(&path),
            _ => unreachable!(),
        };
        match req.call() {
            Ok(response) => {
                if let Ok(json) = response.into_json::<serde_json::Value>() {
                    let json: String = serde_json::to_string_pretty(&json).unwrap();
                    buf.set_text(&json);
                    fill_style_buffer(&mut sbuf, &json);
                    status.set_label("200 OK");
                    status.set_label_color(enums::Color::Green.darker());
                } else {
                    dialog::message_default("Error parsing json");
                }
            }
            Err(Error::Status(code, response)) => {
                status.set_label(&format!("{} {}", code, response.status_text()));
                status.set_label_color(enums::Color::Red);
            }
            Err(e) => {
                dialog::message_default(&e.to_string());
            }
        }
    });
    a.run().unwrap();
}

fn fill_style_buffer(sbuf: &mut text::TextBuffer, s: &str) {
    let mut local_buf = vec!['A' as u8; s.len()];
    for token in Lexer::new(s.bytes(), BufferType::Span) {
        let mut start: usize = 0;
        let mut last: usize = 0;
        if let Buffer::Span(Span { first, end }) = token.buf {
            start = first as _;
            last = end as _;
        }
        match token.kind {
            TokenType::CurlyOpen => {
                local_buf[start..last].copy_from_slice("A".repeat(last - start).as_bytes())
            }
            TokenType::String => {
                local_buf[start..last].copy_from_slice("B".repeat(last - start).as_bytes())
            }
            TokenType::CurlyClose => {
                local_buf[start..last].copy_from_slice("A".repeat(last - start).as_bytes())
            }
            TokenType::BracketOpen => {
                local_buf[start..last].copy_from_slice("A".repeat(last - start).as_bytes())
            }
            TokenType::BracketClose => {
                local_buf[start..last].copy_from_slice("A".repeat(last - start).as_bytes())
            }
            TokenType::Colon => {
                local_buf[start..last].copy_from_slice("A".repeat(last - start).as_bytes())
            }
            TokenType::Comma => {
                local_buf[start..last].copy_from_slice("A".repeat(last - start).as_bytes())
            }
            TokenType::BooleanTrue => {
                local_buf[start..last].copy_from_slice("C".repeat(last - start).as_bytes())
            }
            TokenType::BooleanFalse => {
                local_buf[start..last].copy_from_slice("C".repeat(last - start).as_bytes())
            }
            TokenType::Number => {
                local_buf[start..last].copy_from_slice("D".repeat(last - start).as_bytes())
            }
            TokenType::Null => {
                local_buf[start..last].copy_from_slice("C".repeat(last - start).as_bytes())
            }
            TokenType::Invalid => {
                local_buf[start..last].copy_from_slice("A".repeat(last - start).as_bytes())
            }
        }
    }
    sbuf.set_text(&String::from_utf8_lossy(&local_buf));
}
