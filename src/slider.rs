use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{block::BlockExt, Block, Widget},
};

pub struct Slider<'a> {
    value: f32,
    min: f32,
    max: f32,
    block: Option<Block<'a>>,
}

impl<'a> Slider<'a> {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        return Self {
            value,
            min,
            max,
            block: None,
        };
    }
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        return self;
    }
    fn render_slider(&self, area: Rect, buf: &mut Buffer) {
        let x_midpoint = (area.left() + area.right()) / 2;
        let y_midpoint = (area.top() + area.bottom()) / 2;

        let px = (area.bottom() - area.top()) as f32 / 2.0;
        let avg = (self.min + self.max) / 2.0;
        let filled = px * ((self.value - avg) / (self.max - avg));

        let top_region = Rect::new(x_midpoint, area.top(), 1, f32::ceil(px - filled) as u16);
        let green_region = Rect::new(x_midpoint, y_midpoint - filled as u16, 1, filled as u16);
        let red_region = Rect::new(x_midpoint, y_midpoint, 1, (-1.0 * filled) as u16);
        let bottom_region = Rect::new(
            x_midpoint,
            y_midpoint + (-1.0 * filled) as u16,
            1,
            px as u16 - ((-1.0 * filled) as u16),
        );

        Block::new()
            .style(Style::default().bg(Color::Indexed(8)))
            .render(top_region, buf);

        Block::new()
            .style(Style::default().bg(Color::Indexed(10)))
            .render(green_region, buf);

        Block::new()
            .style(Style::default().bg(Color::Indexed(12)))
            .render(red_region, buf);

        Block::new()
            .style(Style::default().bg(Color::Indexed(8)))
            .render(bottom_region, buf);

        let handle_region = Rect::new(x_midpoint - 1, (y_midpoint as f32 - filled - 1.0) as u16, 3, 2);
        Block::bordered().style(Style::default().fg(Color::Indexed(1)).bg(Color::Indexed(8))).render(handle_region, buf);
    }
}

impl Widget for Slider<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.block.render(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_slider(inner, buf);
    }
}
