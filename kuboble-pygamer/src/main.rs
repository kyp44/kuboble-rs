#![no_std]
#![no_main]
#![feature(let_chains)]

use rtic_monotonics::systick::prelude::*;

use output::{DisplayDriver, NeoPixels};
use pygamer::{
    hal::{clock::GenericClockController, delay::Delay, prelude::*, timer::TimerCounter},
    Pins, RedLed,
};

mod controls;
mod output;

systick_monotonic!(Mono, 1000);

#[rtic::app(device = pygamer::pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        red_led: RedLed,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        // Get the peripherals
        let mut peripherals = cx.device;
        let _core = cx.core;

        // Setup the clocks
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.gclk,
            &mut peripherals.mclk,
            &mut peripherals.osc32kctrl,
            &mut peripherals.oscctrl,
            &mut peripherals.nvmctrl,
        );
        let pins = Pins::new(peripherals.port).split();

        // Initialize the display
        let mut delay = Delay::new(_core.SYST, &mut clocks);
        let (display, _backlight) = pins
            .display
            .init(
                &mut clocks,
                peripherals.sercom4,
                &mut peripherals.mclk,
                peripherals.tc2,
                &mut delay,
            )
            .unwrap();

        // Start the monotonic
        Mono::start(delay.free(), 120_000_000);

        // Set up the red LED
        let red_led = pins.led_pin.into();

        // Configure a clock for the TC4 and TC5 peripherals
        let timer_clock = clocks.gclk0();
        let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();

        // Set up the neo-pixels driver
        let neopixels_timer = TimerCounter::tc4_(tc45, peripherals.tc4, &mut peripherals.mclk);
        let neopixels = ws2812_timer_delay::Ws2812::new(
            neopixels_timer,
            pins.neopixel.neopixel.into_push_pull_output(),
        );

        display_test::spawn(display).ok().unwrap();
        neopixels_test::spawn(neopixels).ok().unwrap();

        (Shared {}, Local { red_led })
    }

    #[task(priority = 1)]
    async fn display_test(cx: display_test::Context, display: DisplayDriver) {
        output::display_test(display).await
    }

    #[task(priority = 1)]
    async fn neopixels_test(cx: neopixels_test::Context, neopixels: NeoPixels) {
        output::neopixels_test(neopixels).await;
    }

    #[idle(local = [red_led])]
    fn idle(cx: idle::Context) -> ! {
        let mut count = 0;
        loop {
            count += 1;
            if count > 500 {
                cx.local.red_led.toggle().unwrap();
                count = 0;
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
