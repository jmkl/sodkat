use crossbeam_channel::{unbounded, Receiver, Sender};
use device_query::Keycode;
use nanoserde::{DeJson, SerJson};
use once_cell::sync::{Lazy, OnceCell};
use std::fmt::Display;

/*************************
MACROS
**************************/
#[macro_export]
macro_rules! filter_key {
    ($key:expr) => {
        match $key {
            Keycode::LControl | Keycode::RControl => None,
            Keycode::LShift | Keycode::RShift => None,
            Keycode::LAlt | Keycode::RAlt => None,
            Keycode::LMeta | Keycode::RMeta => None,
            _ => Some(*$key),
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SodKatState {
    Press,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SerJson, DeJson)]
pub enum SodKatModifier {
    ALT,
    SHIFT,
    CONTROL,
    META,
}
pub trait FormatModifiers {
    fn to_vec(&self) -> Vec<String>;
    fn format(&self) -> String;
}

// Implement `FormatModifiers` for `Vec<SodKatModifier>`
impl FormatModifiers for Vec<SodKatModifier> {
    fn to_vec(&self) -> Vec<String> {
        self.iter()
            .map(|modifier| match *modifier {
                SodKatModifier::ALT => "Alt".into(),
                SodKatModifier::SHIFT => "Shift".into(),
                SodKatModifier::CONTROL => "Ctrl".into(),
                SodKatModifier::META => "Meta".into(),
            })
            .collect::<Vec<String>>()
    }
    fn format(&self) -> String {
        let mut r = self
            .iter()
            .map(|modifier| match *modifier {
                SodKatModifier::ALT => "A",
                SodKatModifier::SHIFT => "S",
                SodKatModifier::CONTROL => "C",
                SodKatModifier::META => "M",
            })
            .collect::<Vec<&str>>();
        r.sort();
        r.join("-")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalSodKatEvent {
    pub keycode: Keycode,
    pub state: SodKatState,
    pub modifier: Vec<SodKatModifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, SerJson)]
pub struct GlobalSodKatPayload {
    pub keycode: String,
    pub is_pressed: bool,
    pub modifier: Vec<String>,
}

impl GlobalSodKatEvent {
    pub fn to_payload(&self) -> GlobalSodKatPayload {
        GlobalSodKatPayload {
            keycode: self.keycode.to_string(),
            is_pressed: self.state == SodKatState::Press,
            modifier: self.modifier.to_vec(),
        }
    }
}

impl Display for GlobalSodKatEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}",
            self.modifier.format(),
            self.keycode.to_string(),
        )
    }
}

type GlobalSodKatEventReceiver = Receiver<GlobalSodKatEvent>;
type GlobalSodKatEventHandler = Box<dyn Fn(GlobalSodKatEvent) + Send + Sync + 'static>;
static GLOBAL_SODKAT_CHANNEL: Lazy<(Sender<GlobalSodKatEvent>, GlobalSodKatEventReceiver)> =
    Lazy::new(unbounded);
static GLOBAL_SODKAT_EVENT_HANDLER: OnceCell<Option<GlobalSodKatEventHandler>> = OnceCell::new();

impl GlobalSodKatEvent {
    pub fn receiver<'a>() -> &'a GlobalSodKatEventReceiver {
        &GLOBAL_SODKAT_CHANNEL.1
    }
    //TODO! remove this
    #[allow(dead_code)]
    pub fn set_event_handler<F: Fn(GlobalSodKatEvent) + Send + Sync + 'static>(f: Option<F>) {
        if let Some(f) = f {
            let _ = GLOBAL_SODKAT_EVENT_HANDLER.set(Some(Box::new(f)));
        } else {
            let _ = GLOBAL_SODKAT_EVENT_HANDLER.set(None);
        }
    }

    pub(crate) fn send(event: GlobalSodKatEvent) {
        if let Some(handler) = GLOBAL_SODKAT_EVENT_HANDLER.get_or_init(|| None) {
            handler(event);
        } else {
            let _ = GLOBAL_SODKAT_CHANNEL.0.send(event);
        }
    }
}
