use crate::{event, filter_key};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;

use event::{GlobalSodKatEvent, SodKatModifier, SodKatState};

pub struct SodKat {
    delay: u64,
}

impl SodKat {
    pub fn new() -> Self {
        Self { delay: 10 }
    }

    fn modkeys(keys: &Vec<Keycode>) -> Vec<SodKatModifier> {
        let mut modifier = vec![];
        for key in keys {
            match key {
                Keycode::LControl | Keycode::RControl => modifier.push(SodKatModifier::CONTROL),
                Keycode::LShift | Keycode::RShift => modifier.push(SodKatModifier::SHIFT),
                Keycode::LAlt | Keycode::RAlt => modifier.push(SodKatModifier::ALT),
                Keycode::LMeta | Keycode::RMeta => modifier.push(SodKatModifier::META),
                _ => {}
            }
        }
        modifier
    }

    pub fn listen(&self) {
        let delay = self.delay.clone();
        thread::spawn(move || {
            let dv = DeviceState::new();
            let mut last_keys = vec![];
            loop {
                let keys = dv.get_keys();
                let modifier: Vec<SodKatModifier> = SodKat::modkeys(&last_keys);
                for key in &keys {
                    if !last_keys.contains(key) {
                        let current_key = filter_key!(key);
                        if let Some(k) = current_key {
                            GlobalSodKatEvent::send(GlobalSodKatEvent {
                                state: SodKatState::Press,
                                keycode: k,
                                modifier: modifier.clone(),
                            });
                        }
                    }
                }

                for key in &last_keys {
                    if !keys.contains(key) {
                        let current_key = filter_key!(key);
                        if let Some(k) = current_key {
                            GlobalSodKatEvent::send(GlobalSodKatEvent {
                                state: SodKatState::Release,
                                keycode: k,
                                modifier: modifier.clone(),
                            });
                        }
                    }
                }
                last_keys = keys;
                thread::sleep(Duration::from_millis(delay));
            }
        });
    }
}
