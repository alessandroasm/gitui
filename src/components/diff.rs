use crate::{
    components::{CommandInfo, Component},
    strings,
};
use asyncgit::{
    hash, sync::diff::Hunk, Diff, DiffLine, DiffLineType,
};
use crossterm::event::{Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Frame,
};

///
#[derive(Default)]
pub struct DiffComponent {
    diff: Diff,
    scroll: u16,
    focused: bool,
    current: (String, bool),
    current_hash: u64,
}

impl DiffComponent {
    ///
    fn can_scroll(&self) -> bool {
        self.diff.0.len() > 1
    }
    ///
    pub fn current(&self) -> (String, bool) {
        (self.current.0.clone(), self.current.1)
    }
    ///
    pub fn clear(&mut self) {
        self.current.0.clear();
        self.diff = Diff::default();
    }
    ///
    pub fn update(
        &mut self,
        path: String,
        is_stage: bool,
        diff: Diff,
    ) {
        let hash = hash(&diff);

        if self.current_hash != hash {
            self.current = (path, is_stage);
            self.current_hash = hash;
            self.diff = diff;
            self.scroll = 0;
        }
    }
    ///
    fn scroll(&mut self, inc: bool) {
        if inc {
            self.scroll =
                self.scroll.checked_add(1).unwrap_or(self.scroll);
        } else {
            self.scroll = self.scroll.checked_sub(1).unwrap_or(0);
        }
    }
}

impl Component for DiffComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let txt = self
            .diff
            .0
            .iter()
            .map(|h: &Hunk| h.0.clone())
            .flatten()
            .map(|e: DiffLine| {
                let content = e.content.clone();
                match e.line_type {
                    DiffLineType::Delete => Text::Styled(
                        content.into(),
                        Style::default()
                            .fg(Color::Red)
                            .bg(Color::Black),
                    ),
                    DiffLineType::Add => Text::Styled(
                        content.into(),
                        Style::default()
                            .fg(Color::Green)
                            .bg(Color::Black),
                    ),
                    DiffLineType::Header => Text::Styled(
                        content.into(),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Gray)
                            .modifier(Modifier::BOLD),
                    ),
                    _ => Text::Raw(content.into()),
                }
            })
            .collect::<Vec<_>>();

        let mut style_border = Style::default();
        let mut style_title = Style::default();
        if self.focused {
            style_border = style_border.fg(Color::Green);
            style_title = style_title.modifier(Modifier::BOLD);
        }

        Paragraph::new(txt.iter())
            .block(
                Block::default()
                    .title(strings::TITLE_DIFF)
                    .borders(Borders::ALL)
                    .border_style(style_border)
                    .title_style(style_title),
            )
            .alignment(Alignment::Left)
            .scroll(self.scroll)
            .render(f, r);
    }

    fn commands(&self) -> Vec<CommandInfo> {
        vec![CommandInfo::new(
            strings::CMD_SCROLL,
            self.can_scroll(),
            self.focused,
        )]
    }

    fn event(&mut self, ev: Event) -> bool {
        if self.focused {
            if let Event::Key(e) = ev {
                // if ev == Event::Key(KeyCode::PageDown.into()) {
                //     self.scroll(true);
                // }
                // if ev == Event::Key(KeyCode::PageUp.into()) {
                //     self.scroll(false);
                // }
                // if let Event::Mouse(MouseEvent::ScrollDown(_, _, _)) = ev
                // {
                //     self.scroll(true);
                // }
                // if let Event::Mouse(MouseEvent::ScrollUp(_, _, _)) = ev {
                //     self.scroll(false);
                // }
                return match e.code {
                    KeyCode::Down => {
                        self.scroll(true);
                        true
                    }
                    KeyCode::Up => {
                        self.scroll(false);
                        true
                    }
                    _ => false,
                };
            }
        }

        false
    }

    ///
    fn focused(&self) -> bool {
        self.focused
    }
    ///
    fn focus(&mut self, focus: bool) {
        self.focused = focus
    }
}