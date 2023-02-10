use pc_keyboard::Ps2Decoder;

fn main() {
    let mut decoder = Ps2Decoder::new();

    // If you get all 11 bits as on `u16`
    match decoder.add_word(0x0402) {
        Ok(byte) => println!("Word 0x0402 is byte 0x{:02x}", byte),
        Err(e) => println!("Word 0x0402 failed to decode: {:?}", e),
    }

    // If you get a bit at a time
    for bit in [
        false, true, false, false, false, false, false, false, false, false, true,
    ] {
        match decoder.add_bit(bit) {
            Ok(None) => println!("Added {}, not enough bits yet!", bit as u8),
            Ok(Some(byte)) => println!("Added {}, got byte 0x{byte:02x}", bit as u8),
            Err(e) => println!("Failed to decode: {e:?}"),
        }
    }

    // Flipped a random bit, so we get a parity error
    for bit in [
        false, true, false, false, false, false, true, false, false, false, true,
    ] {
        match decoder.add_bit(bit) {
            Ok(None) => println!("Added {}, not enough bits yet!", bit as u8),
            Ok(Some(byte)) => println!("Added {}, got byte 0x{byte:02x}", bit as u8),
            Err(e) => println!("Failed to decode: {e:?}"),
        }
    }
}
