use clap::builder::{
    StyledStr, Styles,
    styling::{AnsiColor, Style},
};

pub(super) fn styles() -> Styles {
    Styles::styled()
        .header(Style::new().bold())
        .usage(Style::new())
        .literal(AnsiColor::Cyan.on_default().bold())
        .placeholder(AnsiColor::White.on_default())
        .invalid(AnsiColor::Cyan.on_default().bold())
}

pub(super) fn about() -> StyledStr {
    use owo_colors::OwoColorize;

    "@ShadowRZ's Nix/NixOS/Nixpkgs helpers".bold().to_string().into()
}
