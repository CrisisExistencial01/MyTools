use std::collections::HashMap;

pub fn default_keybindings() -> HashMap<String, String> {
    HashMap::from([
        ("q".to_string(), "Quit".to_string()),
        ("k".to_string(), "SelectUp".to_string()),
        ("j".to_string(), "SelectDown".to_string()),
        ("tab".to_string(), "NextPanel".to_string()),
        ("backtab".to_string(), "PrevPanel".to_string()),
        ("?".to_string(), "ToggleHelp".to_string()),
        ("enter".to_string(), "ToggleDetails".to_string()),
        ("/".to_string(), "OpenPalette".to_string()),
        ("esc".to_string(), "ClosePalette".to_string()),
        ("f".to_string(), "OpenFilter".to_string()),
    ])
}
