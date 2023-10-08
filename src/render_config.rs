use inquire::{
    error::InquireResult,
    ui::{Color, RenderConfig, StyleSheet, Styled},
};

pub fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("?").with_fg(Color::DarkGreen);
    render_config.highlighted_option_prefix = Styled::new("❯").with_fg(Color::LightYellow);

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new().with_fg(Color::LightCyan);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}

pub fn setup_inquire() -> InquireResult<()> {
    inquire::set_global_render_config(get_render_config());
    Ok(())
}
