use core::cmp::{max, min};
use hal::hal::spi::FullDuplex;
use smart_leds::SmartLedsWrite;
use smart_leds::RGB8;
use ws2812_spi::Ws2812;

use crate::hal::time::Hertz;

const LEDS: usize = 12;
pub const FPS: Hertz = Hertz(50);

pub enum RingEvent {
    Mode,
    Plus,
    Minus,
}

pub struct RGBRing<SPI> {
    link: Ws2812<SPI>,
    mode: Mode,
    frame: u32,
    angle: usize,
    sectors: usize,
    color_temp: u8,
    brightness: u8,
    anim: Animation,
}

#[derive(Clone, Copy)]
enum Mode {
    Brightness,
    Sector,
    Rotation,
    ColorTemp,
}

#[derive(Clone, Copy)]
enum Animation {
    Main,
    Ignition,
    ModeIntro(Mode),
}

impl<SPI> RGBRing<SPI>
where
    SPI: FullDuplex<u8>,
{
    pub fn new(spi: SPI) -> RGBRing<SPI> {
        RGBRing {
            link: Ws2812::new(spi),
            mode: Mode::Brightness,
            angle: 0,
            sectors: LEDS,
            brightness: 64,
            color_temp: 128,
            frame: 0,
            anim: Animation::Ignition,
        }
    }

    pub fn handle_event(&mut self, ev: RingEvent) {
        match self.mode {
            Mode::Brightness => match ev {
                RingEvent::Plus => self.brightness = self.brightness.saturating_add(16),
                RingEvent::Minus => self.brightness = max(16, self.brightness.saturating_sub(16)),
                RingEvent::Mode => self.set_mode(Mode::Sector),
            },
            Mode::Sector => match ev {
                RingEvent::Plus => self.sectors = min(LEDS, self.sectors + 1),
                RingEvent::Minus => self.sectors = max(1, self.sectors.saturating_sub(1)),
                RingEvent::Mode => match self.sectors {
                    LEDS => self.set_mode(Mode::ColorTemp),
                    _ => self.set_mode(Mode::Rotation),
                },
            },
            Mode::Rotation => match ev {
                RingEvent::Plus => self.angle = (self.angle + 1) % LEDS,
                RingEvent::Minus => {
                    self.angle = if self.angle == 0 {
                        LEDS - 1
                    } else {
                        self.angle - 1
                    }
                }
                RingEvent::Mode => self.set_mode(Mode::ColorTemp),
            },
            Mode::ColorTemp => match ev {
                RingEvent::Plus => self.color_temp = self.color_temp.saturating_add(16),
                RingEvent::Minus => self.color_temp = self.color_temp.saturating_sub(16),
                RingEvent::Mode => self.set_mode(Mode::Brightness),
            },
        }
    }

    pub fn refresh(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        let mut buffer = [RGB8::default(); LEDS];
        let proto = calculate_color(self.brightness, self.color_temp);
        match &self.anim {
            Animation::Main => {
                for idx in 0..self.sectors {
                    buffer[(self.angle + idx) % LEDS] = proto;
                }
            }
            Animation::Ignition => {
                for (idx, color) in buffer.iter_mut().enumerate() {
                    let val = (self.frame as u8 / 4 + idx as u8) / 6;
                    *color = RGB8 {
                        r: val + 1,
                        g: val,
                        b: val + 1,
                    };
                }
                if self.frame == 60 {
                    self.anim = Animation::Main;
                }
            }
            Animation::ModeIntro(mode) => {
                match mode {
                    Mode::Brightness => {
                        for color in buffer.iter_mut() {
                            *color = calculate_color(self.frame as u8 * 4 % 64, self.color_temp);
                        }
                    }
                    Mode::Sector => {
                        if self.frame <= 24 {
                            let offset = self.frame as usize / 2;
                            for color in buffer.iter_mut().skip(offset) {
                                *color = proto;
                            }
                        } else {
                            let offset = LEDS - ((self.frame as usize - 24) / 2);
                            for color in buffer.iter_mut().skip(offset) {
                                *color = proto;
                            }
                        }
                    }
                    Mode::Rotation => {
                        if self.frame <= 24 {
                            let offset = self.frame as usize / 2;
                            for color in buffer.iter_mut().skip(offset).take(1) {
                                *color = proto;
                            }
                        } else {
                            let offset = LEDS - ((self.frame as usize - 24) / 2);
                            for color in buffer.iter_mut().skip(offset).take(1) {
                                *color = proto;
                            }
                        }
                    }
                    Mode::ColorTemp => {
                        for color in buffer.iter_mut() {
                            let delta = self.frame as u8 * 6;
                            let color_temp = self.color_temp.saturating_add(delta as u8);
                            *color = calculate_color(self.brightness, color_temp);
                        }
                    }
                }
                if self.frame == 42 {
                    self.anim = Animation::Main;
                }
            }
        }
        self.link.write(buffer.iter().cloned()).ok();
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.start_animation(Animation::ModeIntro(mode));
    }

    fn start_animation(&mut self, anim: Animation) {
        self.frame = 0;
        self.anim = anim;
    }
}

fn calculate_color(brightness: u8, color_temp: u8) -> RGB8 {
    let brightness = brightness / 2;
    if color_temp == 128 {
        return RGB8 {
            r: brightness,
            g: brightness,
            b: brightness,
        };
    }
    let brightness = brightness as i16;
    let color_delta = brightness * (128 - color_temp as i16) / 64;
    let r = brightness + (color_delta / 3);
    let g = brightness + (color_delta / 5);
    let b = brightness - (color_delta / 8);
    RGB8 {
        r: r as u8,
        g: g as u8,
        b: b as u8,
    }
}
