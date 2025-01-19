use crate::event::{FormatModifiers, GlobalSodKatEvent};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    sync::Mutex,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SodKatKey {
    pub key: String,
    pub scope: Vec<String>,
    pub function: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setting {
    pub app_name: String,
    pub version: String,
    pub keys: Vec<SodKatKey>,
}

#[derive(Debug)]
struct ComboState {
    combo: bool,
    key: String,
}

impl ComboState {
    fn set(&mut self, key: String) {
        self.combo = true;
        self.key = key;
    }
    fn reset(&mut self) {
        self.combo = false;
        self.key = "".to_string();
    }
}

static COMBO_KEY: Lazy<Mutex<ComboState>> = Lazy::new(|| {
    Mutex::new(ComboState {
        combo: false,
        key: "".to_string(),
    })
});

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
                let default_config =
                    toml::to_string(&Setting::default()).expect("Error writing default config");
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
        let setting: Setting = toml::from_str(&content).unwrap();
        Self {
            app_name: setting.app_name,
            version: setting.version,
            keys: setting.keys,
        }
    }

    fn filter_scope(&self, window_name: &String, scope: &Vec<String>) -> bool {
        window_name.is_empty() || scope.is_empty() || scope.contains(&window_name)
    }
    fn split_code<'a>(&self, input: &'a str) -> Vec<&'a str> {
        input.split(" ").collect()
    }
    pub fn find(&self, ev: &GlobalSodKatEvent, window_name: String) -> Option<&SodKatKey> {
        let keys = ev.tokenize();
        let mut multikey = COMBO_KEY.lock().unwrap();

        if multikey.combo {
            let filtered_keys: Vec<&SodKatKey> = self
                .keys
                .iter()
                .filter(|k| {
                    let formatkey = format!("{} {}", multikey.key, keys);

                    k.key == formatkey
                })
                .collect();
            multikey.reset();
            println!("multikey reset");
            return filtered_keys.get(0).map(|&k| k);
        } else {
            let filtered_keys: Vec<&SodKatKey> = self
                .keys
                .iter()
                .filter(|k| {
                    let split_key = self.split_code(&k.key);
                    let scope = self.filter_scope(&window_name, &k.scope);
                    if !scope {
                        return false;
                    }
                    if split_key.len() > 1 {
                        let val = keys == split_key.get(0).unwrap().to_string();
                        if val {
                            multikey.set(keys.clone());
                        }
                        val
                    } else {
                        k.key == keys
                    }
                })
                .collect();
            if multikey.combo {
                println!("multikey state");
                return None;
            } else {
                return filtered_keys.get(0).map(|&k| k);
            }
        }
    }
}
