// /// Represents a key event.
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[derive(Debug, PartialOrd, Clone, Copy)]
// pub struct KeyEvent {
//     /// The key itself.
//     pub code: KeyCode,
//     /// Additional key modifiers.
//     pub modifiers: KeyModifiers,
// }
//
// /// Represents a key.
// #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// pub enum KeyCode {
//     /// Backspace key.
//     Backspace,
//     /// Enter key.
//     Enter,
//     /// Left arrow key.
//     Left,
//     /// Right arrow key.
//     Right,
//     /// Up arrow key.
//     Up,
//     /// Down arrow key.
//     Down,
//     /// Home key.
//     Home,
//     /// End key.
//     End,
//     /// Page up key.
//     PageUp,
//     /// Page dow key.
//     PageDown,
//     /// Tab key.
//     Tab,
//     /// Shift + Tab key.
//     BackTab,
//     /// Delete key.
//     Delete,
//     /// Insert key.
//     Insert,
//     /// F key. `KeyCode::F(1)` represents F1 key, etc.
//     F(u8),
//     /// A character. `KeyCode::Char('c')` represents `c` character, etc.
//     Char(char),
//     /// Null.
//     Null,
//     /// Escape key.
//     Esc,
// }