extern crate sodkat;
use device_query::Keycode;
use std::{io::Result, thread::sleep};

use sodkat::event::{GlobalSodKatEvent, SodKatState};
use sodkat::setting::Setting;
use sodkat::sodkat::SodKat;
use sodkat::win::get_foreground_app;

use std::time::Duration;

fn spawn_events() -> Result<()> {
    let setting = Setting::new("test.toml");
    let sodkat = SodKat::new();
    sodkat.listen();
    let receiver = GlobalSodKatEvent::receiver();

    loop {
        if let Ok(ev) = receiver.try_recv() {
            if ev.state == SodKatState::Release {
                let scope = match get_foreground_app() {
                    Some(win) => win,
                    None => "".to_string(),
                };
                if ev.keycode == Keycode::Escape {
                    break;
                }
                if let Some(sk) = setting.find(&ev, scope) {
                    println!("{sk:?}");
                }
            }
        }
        sleep(Duration::from_millis(10));
    }
    Ok(())
}

fn main() -> Result<()> {
    spawn_events()?;
    Ok(())
}
