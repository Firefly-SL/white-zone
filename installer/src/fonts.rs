use eframe::egui::{FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle};

// this works and i learned quite about fonts

pub fn configure_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = FontDefinitions::default();

    // regular font
    fonts.font_data.insert(
        "Poppins-Regular".into(),
        FontData::from_static(include_bytes!("../../asset/fonts/Poppins-Regular.ttf")).into(),
    );
    
    // light font
    fonts.font_data.insert(
        "Poppins-Light".into(),
        FontData::from_static(include_bytes!("../../asset/fonts/Poppins-Light.ttf")).into(),
    );

    // assign defualt font into family (tbh idk whats happening in here)
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "Poppins-Regular".into());
    
    // in this we are telling to use Regular as default font and SemiBold as when we call it.
    fonts.families.insert(FontFamily::Proportional, vec!["Poppins-Regular".to_owned()]);
    fonts.families.insert(FontFamily::Name("Light".into()), vec!["Poppins-Light".to_owned()]);
    
    // other stuffs to back it up
    let mut style: Style = (*ctx.style()).clone();
    
    style.text_styles.insert(TextStyle::Body, FontId::new(16.0, FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Button, FontId::new(14.0, FontFamily::Name("Light".into())));
    
    ctx.set_fonts(fonts);
    ctx.set_style(style);
}   