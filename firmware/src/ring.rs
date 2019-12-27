use core::cmp::{max, min};
use hal::hal::spi::FullDuplex;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::SmartLedsWrite;
use smart_leds::RGB8;
use ws2812_spi::Ws2812;

use crate::hal::time::Hertz;

const LEDS: usize = 12;
pub const FPS: Hertz = Hertz(60);

pub enum RingEvent {
    Mode,
    Plus,
    Minus,
}

pub struct RGBRing<SPI> {
    link: Ws2812<SPI>,
    mode: Mode,
    rgb: RGB8,
    color: Hsv,
    frame: u32,
    angle: usize,
    sectors: usize,
    anim: Animation,
}

#[derive(Clone, Copy)]
enum Mode {
    Brightness,
    Hue,
    Saturation,
    Sector,
    Rotation,
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
        let color = Hsv {
            hue: 0,
            sat: 0,
            val: 32,
        };
        RGBRing {
            color,
            rgb: hsv2rgb(color),
            link: Ws2812::new(spi),
            mode: Mode::Brightness,
            angle: 0,
            sectors: 12,
            frame: 0,
            anim: Animation::Ignition,
        }
    }

    pub fn handle_event(&mut self, ev: RingEvent) {
        match self.mode {
            Mode::Brightness => match ev {
                RingEvent::Plus => self.color.val = self.color.val.saturating_add(10),
                RingEvent::Minus => self.color.val = max(10, self.color.val.saturating_sub(10)),
                RingEvent::Mode => self.set_mode(Mode::Hue),
            },
            Mode::Hue => match ev {
                RingEvent::Plus => self.color.hue = self.color.hue.wrapping_add(10),
                RingEvent::Minus => self.color.hue = self.color.hue.wrapping_sub(10),
                RingEvent::Mode => self.set_mode(Mode::Saturation),
            },
            Mode::Saturation => match ev {
                RingEvent::Plus => self.color.sat = self.color.sat.saturating_add(10),
                RingEvent::Minus => self.color.sat = max(10, self.color.sat.saturating_sub(10)),
                RingEvent::Mode => self.set_mode(Mode::Sector),
            },
            Mode::Sector => match ev {
                RingEvent::Plus => self.sectors = min(LEDS, self.sectors + 1),
                RingEvent::Minus => self.sectors = max(1, self.sectors.saturating_sub(1)),
                RingEvent::Mode => match self.sectors {
                    LEDS => self.set_mode(Mode::Brightness),
                    _ => self.set_mode(Mode::Rotation),
                },
            },
            Mode::Rotation => match ev {
                RingEvent::Plus => self.angle = (self.angle + 1) % LEDS,
                RingEvent::Minus => self.angle = self.angle.wrapping_sub(1) % LEDS,
                RingEvent::Mode => self.set_mode(Mode::Brightness),
            },
        }
        self.rgb = hsv2rgb(self.color);
    }

    pub fn refresh(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        let mut buffer = [RGB8::default(); LEDS];
        match &self.anim {
            Animation::Main => {
                for idx in 0..self.sectors {
                    buffer[(self.angle + idx) % LEDS] = self.rgb;
                }
            }
            Animation::Ignition => {
                for (idx, color) in buffer.iter_mut().enumerate() {
                    let val = (self.frame as u8 + idx as u8) / 16;
                    *color = hsv2rgb(Hsv {
                        val,
                        hue: 210,
                        sat: 100 - self.frame as u8,
                    });
                }
                if self.frame == 60 {
                    self.anim = Animation::Main;
                }
            }
            Animation::ModeIntro(mode) => {
                match mode {
                    Mode::Brightness => {
                        for color in buffer.iter_mut() {
                            let val = (self.frame as u8 / 2) % 20;
                            *color = RGB8 {
                                r: val,
                                g: val,
                                b: val,
                            };
                        }
                    }
                    Mode::Hue => {
                        for color in buffer.iter_mut() {
                            *color = hsv2rgb(Hsv {
                                sat: 255 - self.frame as u8,
                                hue: self.frame as u8 * 4,
                                val: 32,
                            });
                        }
                    }
                    Mode::Saturation => {
                        for color in buffer.iter_mut() {
                            *color = hsv2rgb(Hsv {
                                hue: 0,
                                sat: 255 - self.frame as u8,
                                val: 32,
                            });
                        }
                    }
                    Mode::Sector => {
                        if self.frame <= 32 {
                            let offset = self.frame as usize / 4;
                            for color in buffer.iter_mut().skip(offset) {
                                *color = self.rgb;
                            }
                        } else {
                            let offset = LEDS - ((self.frame as usize - 32) / 4);
                            for color in buffer.iter_mut().skip(offset) {
                                *color = self.rgb;
                            }
                        }
                    }
                    Mode::Rotation => {
                        if self.frame <= 32 {
                            let offset = self.frame as usize / 4;
                            for color in buffer.iter_mut().skip(offset).take(1) {
                                *color = self.rgb;
                            }
                        } else {
                            let offset = LEDS - ((self.frame as usize - 32) / 4);
                            for color in buffer.iter_mut().skip(offset).take(1) {
                                *color = self.rgb;
                            }
                        }
                    }
                }
                if self.frame == 64 {
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
