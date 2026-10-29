use crate::widgets::{self, Page};
use crate::{cmd, live, prefs, window};
use gtk::prelude::*;

const DESKTOP: &str = "io.github.design_nexus.nyx.Torrents.desktop";

fn is_default() -> bool {
    cmd::output(&["xdg-mime", "query", "default", "x-scheme-handler/magnet"]).is_some_and(|v| v.trim() == DESKTOP)
}

pub fn build(page: &Page) {
    let p = prefs::get();
    // ----- Behaviour -----
    let g = page.group("Behavior");
    let (r, _) = widgets::switch_row(
        "Keep running when closed",
        "Closing the window keeps torrents going. Run <tt>torrents</tt> to bring it back; <tt>Ctrl+Q</tt> quits.",
        p.keep_running,
        |on| prefs::update(|p| p.keep_running = on),
    );
    g.add(&r);
    let (r, _) = widgets::switch_row(
        "Notify when a download finishes",
        "A desktop notification, or a toast while this window is in front.",
        p.notify_finished,
        |on| prefs::update(|p| p.notify_finished = on),
    );
    g.add(&r);
    let (r, _) = widgets::switch_row("Speeds in bits", "Show Mb/s like internet plans do, instead of MiB/s.", p.net_bits, |on| {
        prefs::update(|p| p.net_bits = on);
        live::poke_all();
    });
    g.add(&r);
    let (r, _) = widgets::choice_row(
        "Speed graph history",
        "How far back the speed graphs reach.",
        widgets::opts(&[("60", "1 minute"), ("300", "5 minutes"), ("600", "10 minutes")]),
        &p.history_secs.to_string(),
        |v| {
            prefs::update(|p| p.history_secs = v.parse().unwrap_or(300));
            live::trim();
        },
    );
    g.add(&r);

    let status = widgets::tag(if is_default() { "Default" } else { "Not default" }, if is_default() { "accent" } else { "" });
    let make_default = gtk::Button::with_label("Make default");
    make_default.set_sensitive(!is_default());
    let controls = widgets::hbox(8);
    controls.append(&status);
    controls.append(&make_default);
    let s2 = status.clone();
    make_default.connect_clicked(move |b| {
        let (b, s2) = (b.clone(), s2.clone());
        cmd::run_async(&["xdg-mime", "default", DESKTOP, "x-scheme-handler/magnet", "application/x-bittorrent"], move |res| {
            match res {
                Ok(_) => {
                    s2.set_text("Default");
                    s2.add_css_class("accent");
                    b.set_sensitive(false);
                    window::toast("Magnet links and .torrent files now open in Torrents.");
                }
                Err(e) => window::toast(&format!("Couldn't set the default: {e}")),
            }
        });
    });
    g.add(&widgets::row(
        "Open magnet links and .torrent files",
        "Make Torrents the app your browser hands them to.",
        Some(controls.upcast_ref()),
    ));
}

/// What the settings card's standard groups are built from.
const STANDARD: nyx_ui::settings_card::Standard = nyx_ui::settings_card::Standard {
    app: "torrents",
    about: nyx_ui::settings_card::About {
        name: "Torrents",
        icon: "io.github.design_nexus.nyx.Torrents",
        version: env!("CARGO_PKG_VERSION"),
        description: "Download and share with BitTorrent, with search through qBittorrent plugins.",
        licence: "MIT",
        website: "https://github.com/design-nexus/nyx-torrents",
        issues: "https://github.com/design-nexus/nyx-torrents/issues",
        credits: &[("libtorrent", "BitTorrent")],
    },
    shortcuts: window::SHORTCUTS,
    reduce_motion: || prefs::get().reduce_motion,
    set_reduce_motion: |on| prefs::update(|p| p.reduce_motion = on),
};

pub fn build_appearance(page: &Page) {
    nyx_ui::settings_card::standard_group(&page.body, "Appearance", &STANDARD);
}

pub fn build_keyboard(page: &Page) {
    nyx_ui::settings_card::standard_group(&page.body, "Keyboard", &STANDARD);
}

pub fn build_about(page: &Page) {
    nyx_ui::settings_card::standard_group(&page.body, "About", &STANDARD);
}
