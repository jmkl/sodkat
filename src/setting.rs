use crate::event::{FormatModifiers, GlobalSodKatEvent};
use nanoserde::{DeJson, SerJson};
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

#[derive(Debug, Clone, DeJson, SerJson)]
pub struct SodKatKey {
    pub key: String,
    pub scope: Vec<String>,
    pub function: String,
}

#[derive(Debug, DeJson, SerJson)]
pub struct Setting {
    pub app_name: String,
    pub version: String,
    pub keys: Vec<SodKatKey>,
}
impl Default for Setting {
    fn default() -> Self {
        Self {
            app_name: "SodKat".to_string(),
            version: "1.0.0".to_string(),
            keys: vec![SodKatKey {
                key: "C-S-A s".to_string(),
                scope: vec![],
                function: "do_something".to_string(),
            }],
        }
    }
}

impl Setting {
    fn write_default(file_path: &str) {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)
        {
            Ok(mut f) => {
                // let default_config =
                // toml::to_string(&Setting::default()).expect("Error writing default config");
                let default_config = SerJson::serialize_json(&Setting::default());
                _ = f.write_all(default_config.as_bytes());
            }
            Err(_) => {
                //file already exists
            }
        }
    }
    pub fn new(file_path: &str) -> Self {
        Self::write_default(file_path);
        let content = match fs::read_to_string(file_path) {
            Ok(result) => result,
            Err(_) => "".to_string(),
        };
        let setting: Setting = DeJson::deserialize_json(&content).unwrap();
        Self {
            app_name: setting.app_name,
            version: setting.version,
            keys: setting.keys,
        }
    }
    fn sort_code(&self, input: &str) -> String {
        if let Some((modifiers, tail)) = input.rsplit_once(' ') {
            let mut modifier_parts: Vec<&str> = modifiers.split('-').collect();
            modifier_parts.sort();
            format!("{} {}", modifier_parts.join("-"), tail)
        } else {
            let mut modifier_parts: Vec<&str> = input.split('-').collect();
            modifier_parts.sort();
            modifier_parts.join("-")
        }
    }
    pub fn find(&self, ev: &GlobalSodKatEvent, window_name: String) -> Option<&SodKatKey> {
        let modifier = ev.modifier.format();
        let formatted_key = format!("{} {}", modifier, ev.keycode.to_string().to_lowercase());
        let formatted_key = formatted_key.trim();
        let s: Vec<&SodKatKey> = self
            .keys
            .iter()
            .filter(|k| {
                let s = self.sort_code(&k.key);
                if window_name.is_empty() || k.scope.is_empty() {
                    formatted_key == s
                } else {
                    if k.scope.contains(&window_name) {
                        formatted_key == s
                    } else {
                        false
                    }
                }
            })
            .collect();
        s.get(0).map(|&k| k)
    }
}
