#![no_std]
#![no_main]
#![feature(let_chains)]

use atsamd_hal::dmac::{ChId, DmaController, PriorityLevel, Ready};
use atsamd_hal::gpio::PA01;
use atsamd_hal::pwm::Pwm2;
use atsamd_hal::{self as hal, pwm};
use controls::PyGamerController;
use core::cell::RefCell;
use embedded_hal_bus::spi::{self as bspi, NoDelay};
use kuboble_core::level_select::LevelProgress;
use output::PyGamerOutput;
use pac::{CorePeripherals, Peripherals};
use pygamer::hal::adc::Adc;
use pygamer::hal::clock::GenericClockController;
use pygamer::hal::delay::Delay;
use pygamer::hal::dmac::Channel;
use pygamer::hal::prelude::*;
use pygamer::hal::sercom::spi;
use pygamer::hal::timer::TimerCounter;
use pygamer::pac::gclk::pchctrl::Genselect;
use pygamer::{entry, pac, Pins, TftCs, TftDc, TftPads, TftReset};
use pygamer_engine::run_game;
use st7735_lcd::{Orientation, ST7735};

mod controls;
mod output;

pub type TftDmaSpi<Id> = bspi::ExclusiveDevice<
    spi::PanicOnRead<
        spi::Spi<spi::Config<TftPads>, spi::Tx, hal::typelevel::NoneT, Channel<Id, Ready>>,
    >,
    TftCs,
    NoDelay,
>;

trait DisplayExt {
    fn init_dma<Id: ChId>(
        self,
        clocks: &mut GenericClockController,
        sercom4: pac::Sercom4,
        channel: Channel<Id, Ready>,
        mclk: &mut pac::Mclk,
        timer2: pac::Tc2,
        delay: &mut pygamer::hal::delay::Delay,
    ) -> Result<(ST7735<TftDmaSpi<Id>, TftDc, TftReset>, Pwm2<PA01>), ()>;
}
impl DisplayExt for pygamer::Display {
    /// Convenience for setting up the on board display with DMA.
    fn init_dma<Id: ChId>(
        self,
        clocks: &mut GenericClockController,
        sercom4: pac::Sercom4,
        channel: Channel<Id, Ready>,
        mclk: &mut pac::Mclk,
        timer2: pac::Tc2,
        delay: &mut pygamer::hal::delay::Delay,
    ) -> Result<(ST7735<TftDmaSpi<Id>, TftDc, TftReset>, Pwm2<PA01>), ()> {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom4_core(&gclk0).ok_or(())?;
        let pads = spi::Pads::default()
            .sclk(self.tft_sclk)
            .data_out(self.tft_mosi);
        let mut tft_cs: TftCs = self.tft_cs.into();
        tft_cs.set_low().ok();
        let tft_spi = bspi::ExclusiveDevice::new_no_delay(
            spi::Config::new(mclk, sercom4, pads, clock.freq())
                .spi_mode(spi::MODE_0)
                .baud(16.MHz())
                .enable()
                .with_tx_channel(channel)
                .into_panic_on_read(),
            tft_cs,
        )
        .map_err(|_| ())?;
        let mut display = st7735_lcd::ST7735::new(
            tft_spi,
            self.tft_dc.into(),
            self.tft_reset.into(),
            true,
            false,
            160,
            128,
        );
        display.init(delay)?;
        display.set_orientation(&Orientation::LandscapeSwapped)?;
        let pwm_clock = &clocks.tc2_tc3(&gclk0).ok_or(())?;
        let pwm_pinout = pwm::TC2Pinout::Pa1(self.tft_backlight);
        let mut pwm2 = Pwm2::new(pwm_clock, 1.kHz(), timer2, pwm_pinout, mclk);
        pwm2.set_duty(pwm2.get_max_duty());
        Ok((display, pwm2))
    }
}

#[entry]
fn main() -> ! {
    // Get the peripherals and pins and setup clocks
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port).split();
    // TODO: use sleeping delay here for battery life? Evidently worth it even for delays of like 50ms
    //let x = SleepingDelay::new();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Setup a DMA channel
    let channels = DmaController::init(peripherals.dmac, &mut peripherals.pm).split();
    let channel = channels.0.init(PriorityLevel::Lvl0);

    // Initialize the display using DMA
    let (display, _backlight) = pins
        .display
        .init_dma(
            &mut clocks,
            peripherals.sercom4,
            channel,
            &mut peripherals.mclk,
            peripherals.tc2,
            &mut delay,
        )
        .unwrap();

    // Need to share the delay
    let delay = RefCell::new(delay);

    // Configure a clock for the TC4 and TC5 peripherals
    let timer_clock = clocks.gclk0();
    let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();

    // Set up the neo-pixels driver started at a 3 MHz rate
    let mut neopixels_timer = TimerCounter::tc4_(tc45, peripherals.tc4, &mut peripherals.mclk);
    _embedded_hal_timer_CountDown::start(
        &mut neopixels_timer,
        3.MHz::<1000000, 1>().into_duration(),
    );
    let neopixels = ws2812_timer_delay::Ws2812::new(
        neopixels_timer,
        pins.neopixel.neopixel.into_push_pull_output(),
    );

    // TODO Need to read and later write this from EEPROM
    let mut level_progress = LevelProgress::default();

    run_game(
        PyGamerController::new(
            &delay,
            Adc::adc1(
                peripherals.adc1,
                &mut peripherals.mclk,
                &mut clocks,
                Genselect::Gclk11,
            ),
            pins.joystick.init(),
            pins.buttons.init(),
        ),
        PyGamerOutput::new(display, neopixels),
        &mut level_progress,
    );

    loop {}
}
