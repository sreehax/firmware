#![no_std]
#![no_main]

/// N-Key Rollover Support
mod nkro;
mod matrix;

use matrix::Matrix;
// The macro for our start-up function
use rp_pico::entry;

use rp_pico::hal::gpio::{DynPinId, FunctionSioOutput, Pin, FunctionSioInput, PullUp, PullNone};
// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;
use nkro::NKROReport;
/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

type OutputType = Pin<DynPinId, FunctionSioOutput, PullNone>;
type InputType = Pin<DynPinId, FunctionSioInput, PullUp>;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Work around the RP2040-E5 Errata
    let mut sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut matrix = Matrix::<OutputType, InputType, 15, 5>::new(
        [
            pins.gpio0
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio1
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio2
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio3
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio4
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio5
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio6
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio7
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio8
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio9
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio10
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio11
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio12
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio13
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio14
                .into_push_pull_output()
                .into_pull_type()
                .into_dyn_pin(),
        ],
        [
            pins.gpio16
                .into_pull_up_input()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio17
                .into_pull_up_input()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio18
                .into_pull_up_input()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio19
                .into_pull_up_input()
                .into_pull_type()
                .into_dyn_pin(),
            pins.gpio20
                .into_pull_up_input()
                .into_pull_type()
                .into_dyn_pin(),
        ]
    ).expect("bruh moment");

    // Might as well set up all the GPIO while we are at it...

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB HID Class Device driver, providing NKRO Reports every 20ms
    let usb_hid = HIDClass::new(bus_ref, NKROReport::desc(), 20);
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet.
        USB_HID = Some(usb_hid);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0xCAFE, 0xBABE))
        .manufacturer("Sreehari Sreedev")
        .product("Wooden Keyboard")
        .serial_number("CUM-SLUT")
        .device_class(0)
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    unsafe {
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
    
    // Press and release the "A" key every 100ms
    loop {
        let _ = matrix.get_raw().expect("bruh moment 2");
        // push_keyboard_state(no_keys).ok().unwrap_or(0);
    }
}

/// Submit a new keyboard report to the USB stack.
/// We do this with interrupts disabled, to avoid a race hazard with the USB IRQ.
fn push_keyboard_state(report: NKROReport) -> Result<usize, usb_device::UsbError> {
    critical_section::with(|_| unsafe {
        // Now interrupts are disabled, grab the global variable and, if
        // available, send it a HID report
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap()
}

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Handle USB request
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_dev.poll(&mut [usb_hid]);
}
