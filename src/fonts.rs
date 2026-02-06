use eframe::egui::{FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle};

// i don't know why but these fonts are fcked up

pub fn configure_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = FontDefinitions::default();

    // regular font
    fonts.font_data.insert(
        "Poppins-Regular".into(),
        FontData::from_static(include_bytes!("fonts/Poppins-Regular.ttf")).into(),
    );

    // bold font
    fonts.font_data.insert(
        "Poppins-Bold".into(),
        FontData::from_static(include_bytes!("fonts/Poppins-Bold.ttf")).into(),
    );

    // assign fonts to families
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "Poppins-Regular".into());

    // assign the bold font specifically to the heading text style
    let mut style: Style = (*ctx.style()).clone();
    style.text_styles.insert(TextStyle::Heading, FontId::new(32.0, FontFamily::Proportional));
    // override the heading font to bold
    fonts.families.insert(FontFamily::Proportional, vec!["Poppins-Bold".into(), "Poppins-Regular".into()]);

    ctx.set_fonts(fonts);
    ctx.set_style(style);
}