use std::collections::HashMap;

pub fn default_keybindings() -> HashMap<String, String> {
    HashMap::from([
        ("q".to_string(), "Quit".to_string()),
        ("up".to_string(), "SelectUp".to_string()),
        ("down".to_string(), "SelectDown".to_string()),
        ("tab".to_string(), "NextPanel".to_string()),
        ("backtab".to_string(), "PrevPanel".to_string()),
    ])
}
