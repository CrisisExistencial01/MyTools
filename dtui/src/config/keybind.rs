use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyBinding {
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        let parts: Vec<&str> = s.split('+').collect();
        let mut modifiers = KeyModifiers::NONE;

        for part in &parts[..parts.len() - 1] {
            modifiers |= match *part {
                "ctrl" => KeyModifiers::CONTROL,
                "alt" => KeyModifiers::ALT,
                "shift" => KeyModifiers::SHIFT,
                _ => continue,
            };
        }

        let code = Self::parse_key_code(parts.last()?)?;
        Some(KeyBinding { code, modifiers })
    }

    fn parse_key_code(s: &str) -> Option<KeyCode> {
        match s {
            "up" => Some(KeyCode::Up),
            "down" => Some(KeyCode::Down),
            "left" => Some(KeyCode::Left),
            "right" => Some(KeyCode::Right),
            "tab" => Some(KeyCode::Tab),
            "backtab" => Some(KeyCode::BackTab),
            "enter" => Some(KeyCode::Enter),
            "esc" => Some(KeyCode::Esc),
            "delete" => Some(KeyCode::Delete),
            "home" => Some(KeyCode::Home),
            "end" => Some(KeyCode::End),
            "pageup" => Some(KeyCode::PageUp),
            "pagedown" => Some(KeyCode::PageDown),
            "space" => Some(KeyCode::Char(' ')),
            c if c.len() == 1 => Some(KeyCode::Char(c.chars().next()?)),
            _ => None,
        }
    }

    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.code == event.code && self.modifiers == event.modifiers
    }
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            write!(f, "Ctrl+")?;
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            write!(f, "Alt+")?;
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) && self.code != KeyCode::BackTab {
            write!(f, "Shift+")?;
        }

        match self.code {
            KeyCode::Char(c) => write!(f, "{c}"),
            KeyCode::Up => write!(f, "↑"),
            KeyCode::Down => write!(f, "↓"),
            KeyCode::Left => write!(f, "←"),
            KeyCode::Right => write!(f, "→"),
            KeyCode::Tab => write!(f, "Tab"),
            KeyCode::BackTab => write!(f, "Shift+Tab"),
            KeyCode::Enter => write!(f, "Enter"),
            KeyCode::Esc => write!(f, "Esc"),
            _ => write!(f, "{:?}", self.code),
        }
    }
}

#[derive(Debug)]
pub struct KeyBindingMap {
    bindings: HashMap<KeyBinding, String>,
    reverse: HashMap<String, KeyBinding>,
}

impl KeyBindingMap {
    pub fn from_config(map: &HashMap<String, String>) -> Self {
        let mut bindings = HashMap::new();
        let mut reverse = HashMap::new();

        for (key_str, action) in map {
            if let Some(kb) = KeyBinding::from_str(key_str) {
                bindings.insert(kb.clone(), action.clone());
                reverse.insert(action.clone(), kb);
            }
        }

        KeyBindingMap { bindings, reverse }
    }

    pub fn lookup(&self, event: &KeyEvent) -> Option<&str> {
        self.bindings
            .iter()
            .find(|(kb, _)| kb.matches(event))
            .map(|(_, action)| action.as_str())
    }

    pub fn get_key_for_action(&self, action: &str) -> Option<String> {
        self.reverse.get(action).map(|kb| kb.to_string())
    }
}
