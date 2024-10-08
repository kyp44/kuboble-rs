#![no_std]
#![no_main]
#![feature(let_chains)]

mod controls;
mod output;

#[rtic::app(device = pygamer::pac, dispatchers = [EVSYS_0])]
mod app {
    use crate::output::{self, DisplayDriver};
    use atsamd_hal::{
        async_hal::timer::{InterruptHandler, TimerFuture},
        bind_interrupts,
    };
    use pygamer::{
        hal::{clock::GenericClockController, delay::Delay, prelude::*, timer::TimerCounter},
        pac::Tc4,
        Pins, RedLed,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        delay: TimerFuture<Tc4>,
        display: DisplayDriver,
        red_led: RedLed,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        // Get the peripherals and pins and setup clocks
        let mut clocks = GenericClockController::with_internal_32kosc(
            cx.device.gclk,
            &mut cx.device.mclk,
            &mut cx.device.osc32kctrl,
            &mut cx.device.oscctrl,
            &mut cx.device.nvmctrl,
        );
        let pins = Pins::new(cx.device.port).split();
        let mut delay = Delay::new(cx.core.SYST, &mut clocks);

        // Initialize the display
        let (display, _backlight) = pins
            .display
            .init(
                &mut clocks,
                cx.device.sercom4,
                &mut cx.device.mclk,
                cx.device.tc2,
                &mut delay,
            )
            .unwrap();

        // Set up the red LED
        let red_led = pins.led_pin.into();

        // Set up the neo-pixels driver
        // Note: This is the non-deprecated way but is jittery as commented in the example
        // here: https://github.com/atsamd-rs/atsamd/blob/master/boards/pygamer/examples/neopixel_rainbow_spi.rs
        // Maybe look back into this later so we don't have to use the deprecated SpinTimer.
        /* let tc4_clock = clocks.tc4_tc5(&clocks.gclk0()).unwrap();
        let mut neopixels_timer = TimerCounter::tc4_(&tc4_clock, peripherals.TC4, &mut peripherals.MCLK);
        neopixels_timer.start(3.mhz());
        let neopixels_timer = SpinTimer::new(4);
        let neopixels = pins.neopixel.init(neopixels_timer, &mut pins.port);*/

        // Bind interrupt to the timer handler
        bind_interrupts!(struct Irq {
            TC4 => InterruptHandler<pygamer::pac::Tc4>;
        });

        // Setup the async timer
        let gclk0 = clocks.gclk0();
        let delay = TimerCounter::tc4_(
            &clocks.tc4_tc5(&gclk0).unwrap(),
            cx.device.tc4,
            &mut cx.device.mclk,
        )
        .into_future(Irq);

        display_test::spawn().ok().unwrap();

        (
            Shared {},
            Local {
                delay,
                display,
                red_led,
            },
        )
    }

    #[task(priority = 1, local = [delay, display])]
    async fn display_test(cx: display_test::Context) {
        output::display_test(cx.local.delay, cx.local.display).await
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
