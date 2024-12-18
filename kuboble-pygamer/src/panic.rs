use core::panic::PanicInfo;
use pygamer::Pins;

// TODO: This could be useful as a feature in `pygamer`, but depends on publishing the `DisplayWriter`.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    // Get the peripherals
    #[cfg(debug_assertions)]
    let mut peripherals = unsafe { pygamer::pac::Peripherals::steal() };
    #[cfg(not(debug_assertions))]
    let peripherals = unsafe { pygamer::pac::Peripherals::steal() };

    let pins = Pins::new(peripherals.port).split();

    // In debug, print the panic message to the display
    #[cfg(debug_assertions)]
    {
        use core::fmt::Write;
        use embedded_graphics::pixelcolor::Rgb565;
        use embedded_graphics::{mono_font, prelude::*, text};
        use pygamer::hal::clock::GenericClockController;
        use pygamer::hal::delay::Delay;
        use pygamer_engine::prelude::*;

        let core = unsafe { pygamer::pac::CorePeripherals::steal() };

        // Setup the clocks
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.gclk,
            &mut peripherals.mclk,
            &mut peripherals.osc32kctrl,
            &mut peripherals.oscctrl,
            &mut peripherals.nvmctrl,
        );

        // Initialize the display
        let mut delay = Delay::new(core.SYST, &mut clocks);
        let (mut display, _backlight) = pins
            .display
            .init(
                &mut clocks,
                peripherals.sercom4,
                &mut peripherals.mclk,
                peripherals.tc2,
                &mut delay,
            )
            .unwrap();

        let style = DisplayTextStyle::new(
            Point::zero(),
            Some(display.size()),
            mono_font::MonoTextStyleBuilder::new()
                .font(&mono_font::ascii::FONT_5X8)
                .text_color(Rgb565::BLACK)
                .background_color(Rgb565::RED)
                .build(),
            text::TextStyleBuilder::new()
                .alignment(text::Alignment::Left)
                .baseline(text::Baseline::Top)
                .build(),
        );

        let _ = write!(DisplayWriter::new(&mut display, &style), "{}", _info);
    }

    // In release, just light the LED
    #[cfg(not(debug_assertions))]
    {
        use pygamer::hal::prelude::*;
        use pygamer::RedLed;

        let mut red_led: RedLed = pins.led_pin.into();
        red_led.set_high().unwrap();
    }

    loop {
        cortex_m::asm::wfi();
    }
}
