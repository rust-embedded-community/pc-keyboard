use pc_keyboard::{
    layouts::{AnyLayout, Uk105Key},
    DecodedKey, EventDecoder, KeyCode, KeyEvent, KeyState,
};

fn main() {
    let mut decoder = EventDecoder::new(
        AnyLayout::Uk105Key(Uk105Key),
        pc_keyboard::HandleControl::Ignore,
    );

    // User presses 'A' on their UK keyboard, gets a lower-case 'a'.
    let decoded_key = decoder.process_keyevent(KeyEvent {
        code: KeyCode::A,
        state: KeyState::Down,
    });
    assert_eq!(Some(DecodedKey::Unicode('a')), decoded_key);
    println!("Got {:?}", decoded_key);

    // User releases 'A' on their UK keyboard
    let decoded_key = decoder.process_keyevent(KeyEvent {
        code: KeyCode::A,
        state: KeyState::Up,
    });
    assert_eq!(None, decoded_key);

    // User presses 'Shift' on their UK keyboard
    let decoded_key = decoder.process_keyevent(KeyEvent {
        code: KeyCode::LShift,
        state: KeyState::Down,
    });
    assert_eq!(None, decoded_key);

    // User presses 'A' on their UK keyboard, now gets a Capital A
    let decoded_key = decoder.process_keyevent(KeyEvent {
        code: KeyCode::A,
        state: KeyState::Down,
    });
    assert_eq!(Some(DecodedKey::Unicode('A')), decoded_key);
    println!("Got {:?}", decoded_key);

    // User releases 'A' on their UK keyboard
    let decoded_key = decoder.process_keyevent(KeyEvent {
        code: KeyCode::A,
        state: KeyState::Up,
    });
    assert_eq!(None, decoded_key);

    // User releases 'Shift' on their UK keyboard
    let decoded_key = decoder.process_keyevent(KeyEvent {
        code: KeyCode::LShift,
        state: KeyState::Up,
    });
    assert_eq!(None, decoded_key);
}
