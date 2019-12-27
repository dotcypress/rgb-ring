#![no_std]
#![no_main]
#![deny(warnings)]

extern crate panic_semihosting;
extern crate rtfm;
extern crate stm32g0xx_hal as hal;

mod ring;

use hal::exti::Event;
use hal::gpio::*;
use hal::prelude::*;
use hal::rcc::{self, PllConfig};
use hal::spi;
use hal::stm32;
use hal::timer;
use ring::{RGBRing, RingEvent};
use rtfm::app;

type LedTimer = timer::Timer<stm32::TIM17>;
type SPIBus = spi::Spi<stm32::SPI1, (spi::NoSck, spi::NoMiso, gpioa::PA12<Input<Floating>>)>;
type Ring = RGBRing<SPIBus>;

#[app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        ring: Ring,
        timer: LedTimer,
        exti: stm32::EXTI,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let pll_cfg = PllConfig::with_hsi(4, 24, 2);
        let rcc_cfg = rcc::Config::pll().pll_cfg(pll_cfg);
        let mut rcc = ctx.device.RCC.freeze(rcc_cfg);

        let port_a = ctx.device.GPIOA.split(&mut rcc);
        let port_b = ctx.device.GPIOB.split(&mut rcc);
        let port_c = ctx.device.GPIOC.split(&mut rcc);
        let port_f = ctx.device.GPIOF.split(&mut rcc);

        let spi = ctx.device.SPI1.spi(
            (spi::NoSck, spi::NoMiso, port_a.pa12),
            spi::MODE_0,
            3.mhz(),
            &mut rcc,
        );

        let mut exti = ctx.device.EXTI;
        port_b.pb0.listen(SignalEdge::Falling, &mut exti);
        port_c.pc14.listen(SignalEdge::Falling, &mut exti);
        port_f.pf2.listen(SignalEdge::Falling, &mut exti);

        let mut timer = ctx.device.TIM17.timer(&mut rcc);
        timer.start(ring::FPS);
        timer.listen();

        let ring = RGBRing::new(spi);
        init::LateResources { exti, timer, ring }
    }

    #[task(binds = TIM17, resources = [timer, ring])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.resources.ring.refresh();
        ctx.resources.timer.clear_irq();
    }

    #[task(binds = EXTI0_1, resources = [exti, ring])]
    fn btn_plus_click(ctx: btn_plus_click::Context) {
        ctx.resources.ring.handle_event(RingEvent::Plus);
        ctx.resources.exti.unpend(Event::GPIO0);
    }

    #[task(binds = EXTI2_3, resources = [exti, ring])]
    fn btn_mode_click(ctx: btn_mode_click::Context) {
        ctx.resources.ring.handle_event(RingEvent::Mode);
        ctx.resources.exti.unpend(Event::GPIO2);
    }

    #[task(binds = EXTI4_15, resources = [exti, ring])]
    fn btn_minus_click(ctx: btn_minus_click::Context) {
        cortex_m::asm::bkpt();
        ctx.resources.ring.handle_event(RingEvent::Minus);
        ctx.resources.exti.unpend(Event::GPIO14);
    }
};
