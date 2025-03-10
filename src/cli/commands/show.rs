use crate::commands::prelude::*;

use crate::hdcanvas::Canvas;
use crate::utility::similar_colors;

use pastel::Format;

pub fn show_color_tty(out: &mut dyn Write, config: &Config, color: &Color) -> Result<()> {
    let checkerboard_size: usize = 20;
    let color_panel_size: usize = 14;

    let color_panel_position: usize = config.padding + (checkerboard_size - color_panel_size) / 2;
    let text_position_x: usize = checkerboard_size + 2 * config.padding;
    let text_position_y: usize = config.padding + 2;

    let mut canvas = Canvas::new(2 * config.padding + checkerboard_size, 55, config.brush);
    canvas.draw_checkerboard(
        config.padding,
        config.padding,
        checkerboard_size,
        checkerboard_size,
        &Color::graytone(0.94),
        &Color::graytone(0.71),
    );
    canvas.draw_rect(
        color_panel_position,
        color_panel_position,
        color_panel_size,
        color_panel_size,
        color,
    );

    canvas.draw_text(
        text_position_y + 0,
        text_position_x,
        &format!("Hex: {}", color.to_rgb_hex_string(true)),
    );
    canvas.draw_text(
        text_position_y + 2,
        text_position_x,
        &format!("RGB: {}", color.to_rgb_string(Format::Spaces)),
    );
    canvas.draw_text(
        text_position_y + 4,
        text_position_x,
        &format!("HSL: {}", color.to_hsl_string(Format::Spaces)),
    );

    canvas.draw_text(text_position_y + 8, text_position_x, "Most similar:");
    let similar = similar_colors(&color);
    for (i, nc) in similar.iter().enumerate().take(3) {
        canvas.draw_text(text_position_y + 10 + 2 * i, text_position_x + 7, nc.name);
        canvas.draw_rect(
            text_position_y + 10 + 2 * i,
            text_position_x + 1,
            2,
            5,
            &nc.color,
        );
    }

    canvas.print(out)
}

pub fn show_color(out: &mut dyn Write, config: &Config, color: &Color) -> Result<()> {
    if config.interactive_mode {
        show_color_tty(out, config, color)?;
    } else {
        writeln!(out, "{}", color.to_hsl_string(Format::NoSpaces))?;
    }

    Ok(())
}

pub struct ShowCommand;

impl ColorCommand for ShowCommand {
    fn run(
        &self,
        out: &mut dyn Write,
        _: &ArgMatches,
        config: &Config,
        color: &Color,
    ) -> Result<()> {
        show_color(out, config, color)
    }
}
