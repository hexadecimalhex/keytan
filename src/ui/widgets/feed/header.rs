use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    text::Line,
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

/// A horizontal bar with some information.
pub struct FeedHeader;

impl FeedHeader {
    pub fn new_with_state() -> (Self, FeedHeaderState) {
        (
            Self,
            FeedHeaderState {
                notification_count: 0,
            },
        )
    }
}

impl StatefulWidget for FeedHeader {
    type State = FeedHeaderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.height < 4 {
            Line::from("Bar must be at least 4 lines high.").render(area, buf);
            return;
        }

        Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Plain)
            .render(area, buf);
        Line::from(state.get_status_text()).render(
            area.inner(Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );
    }
}

pub struct FeedHeaderState {
    notification_count: usize,
}

impl FeedHeaderState {
    pub fn get_status_text(&self) -> String {
        format!("{} notifications", self.notification_count)
    }
}
