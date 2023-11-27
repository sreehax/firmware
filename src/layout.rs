use crate::keycodes::{KeyCode, KeyCode::*};
use crate::nkro::NKROReport;

type Layout = [[KeyCode; 15]; 5];

pub const MAIN_LAYER: Layout = [
    [Escape, Kb1, Kb2, Kb3, Kb4, Kb5, Kb6, Kb7, Kb8, Kb9, Kb0, Minus, Equal, Bslash, Grave],
    [Tab, Q, W, E, R, T, Y, U, I, O, P, LBracket, RBracket, BSpace, No],
    [LCtrl, A, S, D, F, G, H, J, K, L, SColon, Quote, Enter, No, No],
    [LShift, Z, X, C, V, B, N, M, Comma, Dot, Slash, RShift, Fn, No, No],
    [LAlt, LGui, No, No, No, Space, No, No, No, RGui, RAlt, No, No, No, No],
];

// Convert the raw bitmap from scanning the matrix into a NKRO Report
pub fn bitmap_to_report(input: &[[bool; 15]; 5]) -> NKROReport {
    let mut report = NKROReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        fake_boot: [0; 6],
        keys: [0; 15],
    };

    for (i, row) in input.iter().enumerate() {
        for (j, key_pressed) in row.iter().enumerate() {
            if *key_pressed {
                let key = MAIN_LAYER[i][j];
                if key != No && key != Fn {
                    // Add this key to the descriptor
                    if key.is_modifier() {
                        report.modifier |= key.as_modifier_bit();
                    } else {
                        let mut bruh = false;
                        for c in report.fake_boot.iter_mut() {
                            if *c == 0 {
                                *c = key as u8;
                                bruh = true;
                                break;
                            } else if *c == key as u8 {
                                bruh = true;
                                break;
                            }
                        }
                        if bruh {
                            for i in 0..5 {
                                report.fake_boot[i] = report.fake_boot[i + 1];
                            }
                            report.fake_boot[5] = key as u8;
                        }
                        report.keys[key as usize >> 3] |= 1 << (key as usize & 0x7);
                    }
                }
            }
        }
    }
    return report;
}
