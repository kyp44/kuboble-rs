#![no_std]
#![no_main]
#![feature(let_chains)]

use rtic_monotonics::systick::prelude::*;

mod controls;
mod output;

systick_monotonic!(Mono, 1000);

#[rtic::app(device = pygamer::pac, dispatchers = [TC0])]
mod app {
    use pygamer::{
        delay::Delay,
        gpio::{
            v2::{Output, PushPull, PA23},
            Pin,
        },
        prelude::*,
        timer::SpinTimer,
        Pins,
    };

    type RedLed = Pin<PA23, Output<PushPull>>;

    use crate::{
        output::{self, DisplayDriver},
        Mono,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        display: DisplayDriver,
        red_led: RedLed,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        // Get the peripherals and pins and setup clocks
        let mut clocks = pygamer::clock::GenericClockController::with_internal_32kosc(
            cx.device.GCLK,
            &mut cx.device.MCLK,
            &mut cx.device.OSC32KCTRL,
            &mut cx.device.OSCCTRL,
            &mut cx.device.NVMCTRL,
        );
        let mut pins = Pins::new(cx.device.PORT).split();
        let mut delay = Delay::new(cx.core.SYST, &mut clocks);

        // Initialize the display
        let (display, _backlight) = pins
            .display
            .init(
                &mut clocks,
                cx.device.SERCOM4,
                &mut cx.device.MCLK,
                cx.device.TC2,
                &mut delay,
                &mut pins.port,
            )
            .unwrap();

        // Start the monotonic
        Mono::start(delay.free(), 120_000_000);

        // Set up the red LED
        let red_led = pins.led_pin.into_open_drain_output(&mut pins.port);

        // Set up the neo-pixels driver
        // Note: This is the non-deprecated way but is jittery as commented in the example
        // here: https://github.com/atsamd-rs/atsamd/blob/master/boards/pygamer/examples/neopixel_rainbow_spi.rs
        // Maybe look back into this later so we don't have to use the deprecated SpinTimer.
        /* let tc4_clock = clocks.tc4_tc5(&clocks.gclk0()).unwrap();
        let mut neopixels_timer = TimerCounter::tc4_(&tc4_clock, peripherals.TC4, &mut peripherals.MCLK);
        neopixels_timer.start(3.mhz());
        let neopixels_timer = SpinTimer::new(4);
        let neopixels = pins.neopixel.init(neopixels_timer, &mut pins.port);*/

        display_test::spawn().unwrap_or_else(|_| panic!());

        (Shared {}, Local { display, red_led })
    }

    #[task(priority = 1, local = [display])]
    async fn display_test(cx: display_test::Context) {
        output::display_test(cx.local.display).await
    }

    #[idle(local = [red_led])]
    fn idle(cx: idle::Context) -> ! {
        let mut count = 0u32;
        let mut led_on = false;

        loop {
            count += 1;
            if count > 500 {
                count = 0;
                led_on = !led_on;

                if led_on {
                    cx.local.red_led.set_high().unwrap();
                } else {
                    cx.local.red_led.set_low().unwrap();
                }
            }
            rtic::export::wfi();
        }
    }
}

/* #[entry]
fn main() -> ! {
    // Get the peripherals and pins and setup clocks
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = Pins::new(peripherals.PORT).split();

    //let x = SleepingDelay::new();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Initialize the display
    let (display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();

    // Need to share the delay
    let delay = RefCell::new(delay);

    // Set up the neo-pixels driver
    // Note: This is the non-deprecated way but is jittery as commented in the example
    // here: https://github.com/atsamd-rs/atsamd/blob/master/boards/pygamer/examples/neopixel_rainbow_spi.rs
    // Maybe look back into this later so we don't have to use the deprecated SpinTimer.
    /* let tc4_clock = clocks.tc4_tc5(&clocks.gclk0()).unwrap();
    let mut neopixels_timer = TimerCounter::tc4_(&tc4_clock, peripherals.TC4, &mut peripherals.MCLK);
    neopixels_timer.start(3.mhz()); */
    let neopixels_timer = SpinTimer::new(4);
    let neopixels = pins.neopixel.init(neopixels_timer, &mut pins.port);

    // TODO Need to read and later write this from EEPROM
    let mut level_progress = LevelProgress::default();

    let mut executor = Executor::new();

    run_game(
        PyGamerController::new(
            &delay,
            Adc::adc1(
                peripherals.ADC1,
                &mut peripherals.MCLK,
                &mut clocks,
                GEN_A::GCLK11,
            ),
            pins.joystick.init(&mut pins.port),
            pins.buttons.init(&mut pins.port),
        ),
        PyGamerOutput::new(display, neopixels),
        &mut level_progress,
    );

    loop {}
}
 */
