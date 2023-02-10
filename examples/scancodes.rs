use pc_keyboard::{KeyEvent, ScancodeSet, ScancodeSet1, ScancodeSet2};

fn main() {
    let mut s = ScancodeSet1::new();
    // [ 0x01 ] means "Pressed Escape" in Set 1
    match s.advance_state(0x01) {
        Ok(Some(KeyEvent { code, state })) => {
            println!("Scancode Set 1 0x01 is KeyCode '{code:?}' KeyState '{state:?}'");
        }
        Ok(None) => {
            println!("This is wrong, we didn't think that was a complete sequence");
        }
        Err(e) => {
            println!("There was an error: {e:?}");
        }
    }
    // [ 0x81 ] means "Released Escape" in Set 1
    match s.advance_state(0x81) {
        Ok(Some(KeyEvent { code, state })) => {
            println!("Scancode Set 1 0x81 is KeyCode '{code:?}' KeyState '{state:?}'");
        }
        Ok(None) => {
            println!("This is wrong, we didn't think that was a complete sequence");
        }
        Err(e) => {
            println!("There was an error: {e:?}");
        }
    }

    let mut s = ScancodeSet2::new();
    // [ 0x01 ] means "Pressed F9" in Set 2
    match s.advance_state(0x01) {
        Ok(Some(KeyEvent { code, state })) => {
            println!("Scancode Set 2 0x01 is KeyCode '{code:?}' KeyState '{state:?}'");
        }
        Ok(None) => {
            println!("This is wrong, we didn't think that was a complete sequence");
        }
        Err(e) => {
            println!("There was an error: {e:?}");
        }
    }
    // [ 0xF0, 0x01 ] means "Released F9" in Set 2
    assert_eq!(Ok(None), s.advance_state(0xF0));
    match s.advance_state(0x01) {
        Ok(Some(KeyEvent { code, state })) => {
            println!("Scancode Set 2 0xF0 0x01 is KeyCode '{code:?}' KeyState '{state:?}'");
        }
        Ok(None) => {
            println!("This is wrong, we didn't think that was a complete sequence");
        }
        Err(e) => {
            println!("There was an error: {e:?}");
        }
    }
}
