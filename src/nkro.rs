use usbd_hid::descriptor::generator_prelude::*;

/// NKROReport describes a report and its companion descriptor that can be
/// used to send keyboard button presses to a host and receive the status of the
/// keyboard LEDs. The specialty of this descriptor is that it has a bitmap in
/// the report that allows for infinite NKRO, while maintaining compatibility with
/// broken BIOS HID implementations that don't request a BOOT keyboard. This is
/// done by marking the space that would normally contain the 6 keys pressed
/// in a BOOT keyboard as padding, while still filling them up as expected.
/// The result is that broken BIOSes will ignore the report descriptor and correctly
/// receive the 6KRO data, while any modern Operating System will read the report
/// descriptor and see the infinite NKRO data.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings constant,variable,absolute] reserved=input;
        };
        (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
            #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
        };
        #[packed_bits 48] #[item_settings constant,array,absolute,no_wrap,linear,preferred,not_null] fake_boot=input;
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0x77) = {
            #[packed_bits 120] #[item_settings data,variable,absolute] keys=input;
        };
    }
)]
#[allow(dead_code)]
pub struct NKROReport {
    pub modifier: u8,
    pub reserved: u8,
    pub leds: u8,
    pub fake_boot: [u8; 6],
    pub keys: [u8; 15]
}

#[allow(dead_code)]
impl NKROReport {
    pub fn new() -> NKROReport {
        NKROReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            fake_boot: [0; 6],
            keys: [0; 15],
        }
    }
}
