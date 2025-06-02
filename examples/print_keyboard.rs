//! Prints a keyboard as a Unicode graphic

use std::collections::HashMap;

use pc_keyboard::{DecodedKey, KeyCode, KeyboardLayout, Modifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyboardKind {
    Ansi,
    Iso,
}

fn main() {
    println!("Keyboard Layouts");
    println!("================");
    println!();
    println!("Us104Key:");
    show_kb(KeyboardKind::Ansi, &pc_keyboard::layouts::Us104Key);
    println!("Uk105Key:");
    show_kb(KeyboardKind::Iso, &pc_keyboard::layouts::Uk105Key);
    println!("Colemak:");
    show_kb(KeyboardKind::Ansi, &pc_keyboard::layouts::Colemak);
}

fn show_kb(kind: KeyboardKind, layout: &dyn KeyboardLayout) {
    let mut modifiers = Modifiers {
        lshift: false,
        rshift: false,
        lctrl: false,
        rctrl: false,
        numlock: true,
        capslock: false,
        lalt: false,
        ralt: false,
        rctrl2: false,
    };
    println!("/// ## Unmodified");
    show_kb_modifiers(kind, layout, &modifiers);

    modifiers.capslock = true;
    println!("/// ## Caps Lock");
    show_kb_modifiers(kind, layout, &modifiers);
    modifiers.capslock = false;

    modifiers.lshift = true;
    println!("/// ## Shifted");
    show_kb_modifiers(kind, layout, &modifiers);
    modifiers.lshift = false;

    modifiers.rctrl = true;
    println!("/// ## Control");
    show_kb_modifiers(kind, layout, &modifiers);
    modifiers.rctrl = false;

    modifiers.ralt = true;
    println!("/// ## AltGr");
    show_kb_modifiers(kind, layout, &modifiers);
    modifiers.ralt = false;

    modifiers.ralt = true;
    modifiers.lshift = true;
    println!("/// ## Shift AltGr");
    show_kb_modifiers(kind, layout, &modifiers);
    modifiers.ralt = false;
    modifiers.lshift = false;
}

fn show_kb_modifiers(kind: KeyboardKind, layout: &dyn KeyboardLayout, modifiers: &Modifiers) {
    let mut map = Map::new(modifiers);
    map.insert("esc", KeyCode::Escape, layout);
    map.insert("oem8", KeyCode::Oem8, layout);
    map.insert("key1", KeyCode::Key1, layout);
    map.insert("key2", KeyCode::Key2, layout);
    map.insert("key3", KeyCode::Key3, layout);
    map.insert("key4", KeyCode::Key4, layout);
    map.insert("key5", KeyCode::Key5, layout);
    map.insert("key6", KeyCode::Key6, layout);
    map.insert("key7", KeyCode::Key7, layout);
    map.insert("key8", KeyCode::Key8, layout);
    map.insert("key9", KeyCode::Key9, layout);
    map.insert("key0", KeyCode::Key0, layout);
    map.insert("oem_minus", KeyCode::OemMinus, layout);
    map.insert("oem_plus", KeyCode::OemPlus, layout);
    map.insert("backspace", KeyCode::Backspace, layout);
    map.insert("numpad_divide", KeyCode::NumpadDivide, layout);
    map.insert("numpad_multiply", KeyCode::NumpadMultiply, layout);
    map.insert("numpad_subtract", KeyCode::NumpadSubtract, layout);
    map.insert("tab", KeyCode::Tab, layout);
    map.insert("oem4", KeyCode::Oem4, layout);
    map.insert("oem6", KeyCode::Oem6, layout);
    map.insert("oem7", KeyCode::Oem7, layout);
    map.insert("delete", KeyCode::Delete, layout);
    map.insert("numpad7", KeyCode::Numpad7, layout);
    map.insert("numpad8", KeyCode::Numpad8, layout);
    map.insert("numpad9", KeyCode::Numpad9, layout);
    map.insert("numpadl", KeyCode::NumpadAdd, layout);
    map.insert("oem1", KeyCode::Oem1, layout);
    map.insert("oem3", KeyCode::Oem3, layout);
    map.insert("enter", KeyCode::Return, layout);
    map.insert("numpad4", KeyCode::Numpad4, layout);
    map.insert("numpad5", KeyCode::Numpad5, layout);
    map.insert("numpad6", KeyCode::Numpad6, layout);
    map.insert("oem_comma", KeyCode::OemComma, layout);
    map.insert("oem_period", KeyCode::OemPeriod, layout);
    map.insert("oem2", KeyCode::Oem2, layout);
    map.insert("numpad1", KeyCode::Numpad1, layout);
    map.insert("numpad2", KeyCode::Numpad2, layout);
    map.insert("numpad3", KeyCode::Numpad3, layout);
    map.insert("numpade", KeyCode::NumpadEnter, layout);
    map.insert("space", KeyCode::Spacebar, layout);
    map.insert("numpad0", KeyCode::Numpad0, layout);
    map.insert("numpad_period", KeyCode::NumpadPeriod, layout);
    map.insert("q", KeyCode::Q, layout);
    map.insert("w", KeyCode::W, layout);
    map.insert("e", KeyCode::E, layout);
    map.insert("r", KeyCode::R, layout);
    map.insert("t", KeyCode::T, layout);
    map.insert("y", KeyCode::Y, layout);
    map.insert("u", KeyCode::U, layout);
    map.insert("i", KeyCode::I, layout);
    map.insert("o", KeyCode::O, layout);
    map.insert("p", KeyCode::P, layout);
    map.insert("a", KeyCode::A, layout);
    map.insert("s", KeyCode::S, layout);
    map.insert("d", KeyCode::D, layout);
    map.insert("f", KeyCode::F, layout);
    map.insert("g", KeyCode::G, layout);
    map.insert("h", KeyCode::H, layout);
    map.insert("j", KeyCode::J, layout);
    map.insert("k", KeyCode::K, layout);
    map.insert("l", KeyCode::L, layout);
    map.insert("z", KeyCode::Z, layout);
    map.insert("x", KeyCode::X, layout);
    map.insert("c", KeyCode::C, layout);
    map.insert("v", KeyCode::V, layout);
    map.insert("b", KeyCode::B, layout);
    map.insert("n", KeyCode::N, layout);
    map.insert("m", KeyCode::M, layout);
    if kind == KeyboardKind::Iso {
        map.insert("oem5", KeyCode::Oem5, layout);
        map.print_iso();
    } else {
        map.print_ansi();
    }
}

struct Map {
    inner: HashMap<&'static str, char>,
    modifiers: Modifiers,
}

impl Map {
    fn new(modifiers: &Modifiers) -> Map {
        Map {
            inner: HashMap::new(),
            modifiers: modifiers.clone(),
        }
    }

    fn insert(&mut self, label: &'static str, keycode: KeyCode, layout: &dyn KeyboardLayout) {
        match layout.map_keycode(
            keycode,
            &self.modifiers,
            pc_keyboard::HandleControl::MapLettersToUnicode,
        ) {
            DecodedKey::Unicode(c) => self.inner.insert(label, c),
            e => {
                panic!("Wanted unicode from {:?}, got {:?}!", keycode, e);
            }
        };
    }

    fn get(&self, label: &'static str) -> String {
        let c = self.inner.get(label).unwrap();
        let c_value = *c as u32;
        if c_value <= 32 || c_value == 0x7F {
            format!("{:04x}", c_value)
        } else {
            format!("{:^4}", c)
        }
    }

    fn print_ansi(&self) {
        let es = self.get("esc");
        let o8 = self.get("oem8");
        let k1 = self.get("key1");
        let k2 = self.get("key2");
        let k3 = self.get("key3");
        let k4 = self.get("key4");
        let k5 = self.get("key5");
        let k6 = self.get("key6");
        let k7 = self.get("key7");
        let k8 = self.get("key8");
        let k9 = self.get("key9");
        let k0 = self.get("key0");
        let om = self.get("oem_minus");
        let ol = self.get("oem_plus");
        let bs = self.get("backspace");
        let nd = self.get("numpad_divide");
        let nm = self.get("numpad_multiply");
        let ns = self.get("numpad_subtract");
        let tb = self.get("tab");
        let o4 = self.get("oem4");
        let o6 = self.get("oem6");
        let o7 = self.get("oem7");
        let de = self.get("delete");
        let n7 = self.get("numpad7");
        let n8 = self.get("numpad8");
        let n9 = self.get("numpad9");
        let nl = self.get("numpadl");
        let o1 = self.get("oem1");
        let o3 = self.get("oem3");
        let en = self.get("enter");
        let n4 = self.get("numpad4");
        let n5 = self.get("numpad5");
        let n6 = self.get("numpad6");
        let oc = self.get("oem_comma");
        let op = self.get("oem_period");
        let o2 = self.get("oem2");
        let n1 = self.get("numpad1");
        let n2 = self.get("numpad2");
        let n3 = self.get("numpad3");
        let ne = self.get("numpade");
        let sp = self.get("space");
        let n0 = self.get("numpad0");
        let np = self.get("numpad_period");

        let kq = self.get("q");
        let kw = self.get("w");
        let ke = self.get("e");
        let kr = self.get("r");
        let kt = self.get("t");
        let ky = self.get("y");
        let ku = self.get("u");
        let ki = self.get("i");
        let ko = self.get("o");
        let kp = self.get("p");
        let ka = self.get("a");
        let ks = self.get("s");
        let kd = self.get("d");
        let kf = self.get("f");
        let kg = self.get("g");
        let kh = self.get("h");
        let kj = self.get("j");
        let kk = self.get("k");
        let kl = self.get("l");
        let kz = self.get("z");
        let kx = self.get("x");
        let kc = self.get("c");
        let kv = self.get("v");
        let kb = self.get("b");
        let kn = self.get("n");
        let km = self.get("m");

        println!(
            r#"///
/// ```text
/// ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
/// │{es}│  │    │    │    │    │  │    │    │    │    │  │    │    │    │    │   │    │    │    │
/// └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
///
/// ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
/// │{o8}│{k1}│{k2}│{k3}│{k4}│{k5}│{k6}│{k7}│{k8}│{k9}│{k0}│{om}│{ol}│{bs}     │  │    │    │    │  │    │{nd}│{nm}│{ns}│
/// ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
/// │{tb} │{kq}│{kw}│{ke}│{kr}│{kt}│{ky}│{ku}│{ki}│{ko}│{kp}│{o4}│{o6}│  {o7}  │  │{de}│    │    │  │{n7}│{n8}│{n9}│    │
/// ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴────────┤  └────┴────┴────┘  ├────┼────┼────┤{nl}│
/// │      │{ka}│{ks}│{kd}│{kf}│{kg}│{kh}│{kj}│{kk}│{kl}│{o1}│{o3}│   {en}     │                    │{n4}│{n5}│{n6}│    │
/// ├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────────────┤       ┌────┐       ├────┼────┼────┼────┤
/// │         │{kz}│{kx}│{kc}│{kv}│{kb}│{kn}│{km}│{oc}│{op}│{o2}│              │       │    │       │{n1}│{n2}│{n3}│    │
/// ├─────┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤{ne}│
/// │     │     │     │             {sp}             │     │     │      │      │  │    │    │    │  │{n0}     │{np}│    │
/// └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
/// ```
///"#
        );
    }

    fn print_iso(&self) {
        let es = self.get("esc");
        let o8 = self.get("oem8");
        let k1 = self.get("key1");
        let k2 = self.get("key2");
        let k3 = self.get("key3");
        let k4 = self.get("key4");
        let k5 = self.get("key5");
        let k6 = self.get("key6");
        let k7 = self.get("key7");
        let k8 = self.get("key8");
        let k9 = self.get("key9");
        let k0 = self.get("key0");
        let om = self.get("oem_minus");
        let ol = self.get("oem_plus");
        let bs = self.get("backspace");
        let nd = self.get("numpad_divide");
        let nm = self.get("numpad_multiply");
        let ns = self.get("numpad_subtract");
        let tb = self.get("tab");
        let o4 = self.get("oem4");
        let o6 = self.get("oem6");
        let o7 = self.get("oem7");
        let de = self.get("delete");
        let n7 = self.get("numpad7");
        let n8 = self.get("numpad8");
        let n9 = self.get("numpad9");
        let nl = self.get("numpadl");
        let o1 = self.get("oem1");
        let o3 = self.get("oem3");
        let en = self.get("enter");
        let n4 = self.get("numpad4");
        let n5 = self.get("numpad5");
        let n6 = self.get("numpad6");
        let oc = self.get("oem_comma");
        let op = self.get("oem_period");
        let o2 = self.get("oem2");
        let n1 = self.get("numpad1");
        let n2 = self.get("numpad2");
        let n3 = self.get("numpad3");
        let ne = self.get("numpade");
        let sp = self.get("space");
        let n0 = self.get("numpad0");
        let np = self.get("numpad_period");
        let o5 = self.get("oem5");

        let kq = self.get("q");
        let kw = self.get("w");
        let ke = self.get("e");
        let kr = self.get("r");
        let kt = self.get("t");
        let ky = self.get("y");
        let ku = self.get("u");
        let ki = self.get("i");
        let ko = self.get("o");
        let kp = self.get("p");
        let ka = self.get("a");
        let ks = self.get("s");
        let kd = self.get("d");
        let kf = self.get("f");
        let kg = self.get("g");
        let kh = self.get("h");
        let kj = self.get("j");
        let kk = self.get("k");
        let kl = self.get("l");
        let kz = self.get("z");
        let kx = self.get("x");
        let kc = self.get("c");
        let kv = self.get("v");
        let kb = self.get("b");
        let kn = self.get("n");
        let km = self.get("m");

        println!(
            r#"///
/// ```text
/// ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
/// │{es}│  │    │    │    │    │  │    │    │    │    │  │    │    │    │    │   │    │    │    │
/// └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
///
/// ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
/// │{o8}│{k1}│{k2}│{k3}│{k4}│{k5}│{k6}│{k7}│{k8}│{k9}│{k0}│{om}│{ol}│{bs}     │  │    │    │    │  │    │{nd}│{nm}│{ns}│
/// ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
/// │{tb} │{kq}│{kw}│{ke}│{kr}│{kt}│{ky}│{ku}│{ki}│{ko}│{kp}│{o4}│{o6}│ {en}   │  │{de}│    │    │  │{n7}│{n8}│{n9}│    │
/// ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┐       │  └────┴────┴────┘  ├────┼────┼────┤{nl}│
/// │      │{ka}│{ks}│{kd}│{kf}│{kg}│{kh}│{kj}│{kk}│{kl}│{o1}│{o3}│{o7}│       │                    │{n4}│{n5}│{n6}│    │
/// ├────┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────┴───────┤       ┌────┐       ├────┼────┼────┼────┤
/// │    │{o5}│{kz}│{kx}│{kc}│{kv}│{kb}│{kn}│{km}│{oc}│{op}│{o2}│              │       │    │       │{n1}│{n2}│{n3}│    │
/// ├────┴┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤{ne}│
/// │     │     │     │             {sp}             │     │     │      │      │  │    │    │    │  │{n0}     │{np}│    │
/// └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
/// ```
///"#
        );
    }
}
