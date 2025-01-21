use ratatui::buffer::Buffer;
use ratatui::layout::{Margin, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap};

pub mod feed;
pub mod page;

#[derive(Clone)]
pub struct Note<'a> {
    pub author: User,
    /// A text element to render as content.
    pub text: Paragraph<'a>,
    /// Whether to highlight this note when rendering it.
    pub selected: bool,
}

impl<'a> Note<'a> {
    pub fn new<'n>(author: User, text: Option<&'n str>) -> Self
    where
        'n: 'a,
    {
        Self {
            author,
            text: text.map_or(
                Paragraph::new("[no text]").wrap(Wrap { trim: false }),
                |text| Paragraph::new(text).wrap(Wrap { trim: false }),
            ),
            selected: false,
        }
    }
}

impl<'a> Widget for Note<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        let header = format!("{} @{}", self.author.name, self.author.username);

        buf.set_line(inner.x, inner.y, &Line::from(header), inner.width);
        let text_area = Rect::new(
            inner.x,
            inner.y + 2,
            inner.width,
            self.text.line_count(inner.width) as u16,
        );
        self.text.render(text_area, buf);

        let borders = if self.selected {
            Borders::ALL
        } else {
            Borders::TOP | Borders::BOTTOM
        };
        Block::new()
            .borders(borders)
            .border_type(BorderType::Plain)
            .render(area, buf);
    }
}

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub username: String,
}
