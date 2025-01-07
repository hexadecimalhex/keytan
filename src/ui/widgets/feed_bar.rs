use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    text::Line,
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

/// A horizontal bar with some information.
pub struct FeedBar;

impl FeedBar {
    pub fn new_with_state() -> (Self, FeedBarState) {
        (
            Self,
            FeedBarState {
                notification_count: 0,
            },
        )
    }
}

impl StatefulWidget for FeedBar {
    type State = FeedBarState;

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

pub struct FeedBarState {
    notification_count: usize,
}

impl FeedBarState {
    pub fn get_status_text(&self) -> String {
        format!("{} notifications", self.notification_count)
    }
}
