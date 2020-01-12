use core::cmp::{max, min};
use hal::hal::spi::FullDuplex;
use smart_leds::SmartLedsWrite;
use smart_leds::RGB8;
use ws2812_spi::Ws2812;

use crate::hal::time::Hertz;
use crate::palette;

const LEDS: usize = 12;
pub const FPS: Hertz = Hertz(50);

pub enum ControlEvent {
    Mode,
    Plus,
    Minus,
}

pub struct RGBRing<SPI> {
    link: Ws2812<SPI>,
    mode: Mode,
    anim: Animation,
    frame: usize,
    luma: usize,
    temp: usize,
    sectors: usize,
    angle: usize,
}

#[derive(Clone, Copy)]
enum Mode {
    Luma,
    Sector,
    Rotation,
    Temp,
}

#[derive(Clone, Copy)]
enum Animation {
    Main,
    Ignition,
    Intro(Mode),
}

impl<SPI> RGBRing<SPI>
where
    SPI: FullDuplex<u8>,
{
    pub fn new(spi: SPI) -> RGBRing<SPI> {
        RGBRing {
            link: Ws2812::new(spi),
            mode: Mode::Luma,
            angle: 0,
            sectors: LEDS,
            luma: 0,
            temp: 5,
            frame: 0,
            anim: Animation::Ignition,
        }
    }

    pub fn handle_event(&mut self, ev: ControlEvent) {
        match self.mode {
            Mode::Luma => match ev {
                ControlEvent::Plus => self.luma = min(7, self.luma + 1),
                ControlEvent::Minus => self.luma = self.luma.saturating_sub(1),
                ControlEvent::Mode => self.switch_to_mode(Mode::Sector),
            },
            Mode::Temp => match ev {
                ControlEvent::Plus => self.temp = min(7, self.temp + 1),
                ControlEvent::Minus => self.temp = self.temp.saturating_sub(1),
                ControlEvent::Mode => self.switch_to_mode(Mode::Luma),
            },
            Mode::Sector => match ev {
                ControlEvent::Plus => self.sectors = min(LEDS, self.sectors + 1),
                ControlEvent::Minus => self.sectors = max(1, self.sectors.saturating_sub(1)),
                ControlEvent::Mode => match self.sectors {
                    LEDS => self.switch_to_mode(Mode::Temp),
                    _ => self.switch_to_mode(Mode::Rotation),
                },
            },
            Mode::Rotation => match ev {
                ControlEvent::Plus => self.angle = (self.angle + 1) % LEDS,
                ControlEvent::Minus => {
                    self.angle = if self.angle == 0 {
                        LEDS - 1
                    } else {
                        self.angle - 1
                    }
                }
                ControlEvent::Mode => self.switch_to_mode(Mode::Temp),
            },
        }
    }

    pub fn refresh(&mut self) {
        self.frame = self.frame.wrapping_add(1);

        let mut buffer = [RGB8::default(); LEDS];
        let proto = palette::get_color(self.luma, self.temp);
        match &self.anim {
            Animation::Main => {
                for idx in 0..self.sectors {
                    buffer[(self.angle + idx) % LEDS] = proto;
                }
            }
            Animation::Ignition => {
                for (idx, color) in buffer.iter_mut().enumerate() {
                    let luma = (self.frame + idx) / 18;
                    let temp = 7 - (self.frame + idx / 2) / 32;
                    *color = palette::get_color(luma, temp);
                }
            }
            Animation::Intro(mode) => match mode {
                Mode::Luma => {
                    for color in buffer.iter_mut() {
                        let luma = (self.frame / 10) % 2;
                        *color = palette::get_color(luma, self.temp);
                    }
                }
                Mode::Sector => {
                    if self.frame <= 20 {
                        let offset = self.frame as usize / 2;
                        for color in buffer.iter_mut().skip(offset) {
                            *color = proto;
                        }
                    } else {
                        let offset = LEDS - ((self.frame as usize - 20) / 2);
                        for color in buffer.iter_mut().skip(offset) {
                            *color = proto;
                        }
                    }
                }
                Mode::Rotation => {
                    if self.frame <= 20 {
                        let offset = self.frame as usize / 2;
                        for color in buffer.iter_mut().skip(offset).take(1) {
                            *color = proto;
                        }
                    } else {
                        let offset = LEDS - ((self.frame as usize - 20) / 2);
                        for color in buffer.iter_mut().skip(offset).take(1) {
                            *color = proto;
                        }
                    }
                }
                Mode::Temp => {
                    for color in buffer.iter_mut() {
                        let temp = (self.frame / 6) % 7 + 1;
                        *color = palette::get_color(self.luma, temp);
                    }
                }
            },
        }
        if self.frame == 40 {
            self.anim = Animation::Main;
        }
        self.link.write(buffer.iter().cloned()).ok();
    }

    fn switch_to_mode(&mut self, mode: Mode) {
        self.anim = Animation::Intro(mode);
        self.frame = 0;
        self.mode = mode;
    }
}
