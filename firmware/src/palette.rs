use smart_leds::RGB8;

pub fn get_color(luma: usize, temp: usize) -> RGB8 {
    PALLETE[7 - luma + temp * 8]
}

const PALLETE: [RGB8; 64] = [
    RGB8 {
        r: 165,
        g: 189,
        b: 255,
    },
    RGB8 {
        r: 128,
        g: 150,
        b: 208,
    },
    RGB8 {
        r: 96,
        g: 114,
        b: 167,
    },
    RGB8 {
        r: 70,
        g: 85,
        b: 131,
    },
    RGB8 {
        r: 48,
        g: 61,
        b: 99,
    },
    RGB8 {
        r: 31,
        g: 40,
        b: 72,
    },
    RGB8 {
        r: 17,
        g: 24,
        b: 49,
    },
    RGB8 { r: 8, g: 13, b: 31 },
    RGB8 {
        r: 176,
        g: 196,
        b: 255,
    },
    RGB8 {
        r: 138,
        g: 155,
        b: 208,
    },
    RGB8 {
        r: 106,
        g: 120,
        b: 167,
    },
    RGB8 {
        r: 78,
        g: 90,
        b: 131,
    },
    RGB8 {
        r: 55,
        g: 64,
        b: 99,
    },
    RGB8 {
        r: 35,
        g: 43,
        b: 72,
    },
    RGB8 {
        r: 21,
        g: 27,
        b: 49,
    },
    RGB8 {
        r: 10,
        g: 14,
        b: 31,
    },
    RGB8 {
        r: 191,
        g: 206,
        b: 255,
    },
    RGB8 {
        r: 151,
        g: 163,
        b: 208,
    },
    RGB8 {
        r: 117,
        g: 128,
        b: 167,
    },
    RGB8 {
        r: 88,
        g: 96,
        b: 131,
    },
    RGB8 {
        r: 63,
        g: 69,
        b: 99,
    },
    RGB8 {
        r: 42,
        g: 47,
        b: 72,
    },
    RGB8 {
        r: 26,
        g: 30,
        b: 49,
    },
    RGB8 {
        r: 14,
        g: 16,
        b: 31,
    },
    RGB8 {
        r: 216,
        g: 220,
        b: 255,
    },
    RGB8 {
        r: 174,
        g: 178,
        b: 208,
    },
    RGB8 {
        r: 137,
        g: 138,
        b: 167,
    },
    RGB8 {
        r: 104,
        g: 106,
        b: 131,
    },
    RGB8 {
        r: 76,
        g: 78,
        b: 99,
    },
    RGB8 {
        r: 53,
        g: 54,
        b: 72,
    },
    RGB8 {
        r: 34,
        g: 35,
        b: 49,
    },
    RGB8 {
        r: 20,
        g: 21,
        b: 31,
    },
    RGB8 {
        r: 255,
        g: 255,
        b: 255,
    },
    RGB8 {
        r: 208,
        g: 208,
        b: 208,
    },
    RGB8 {
        r: 167,
        g: 165,
        b: 167,
    },
    RGB8 {
        r: 131,
        g: 129,
        b: 131,
    },
    RGB8 {
        r: 99,
        g: 98,
        b: 99,
    },
    RGB8 {
        r: 72,
        g: 70,
        b: 72,
    },
    RGB8 {
        r: 49,
        g: 48,
        b: 49,
    },
    RGB8 {
        r: 31,
        g: 31,
        b: 31,
    },
    RGB8 {
        r: 255,
        g: 235,
        b: 216,
    },
    RGB8 {
        r: 208,
        g: 191,
        b: 174,
    },
    RGB8 {
        r: 167,
        g: 151,
        b: 137,
    },
    RGB8 {
        r: 131,
        g: 116,
        b: 104,
    },
    RGB8 {
        r: 99,
        g: 86,
        b: 76,
    },
    RGB8 {
        r: 72,
        g: 62,
        b: 53,
    },
    RGB8 {
        r: 49,
        g: 40,
        b: 34,
    },
    RGB8 {
        r: 31,
        g: 25,
        b: 20,
    },
    RGB8 {
        r: 255,
        g: 214,
        b: 181,
    },
    RGB8 {
        r: 208,
        g: 170,
        b: 143,
    },
    RGB8 {
        r: 167,
        g: 134,
        b: 110,
    },
    RGB8 {
        r: 131,
        g: 102,
        b: 81,
    },
    RGB8 {
        r: 99,
        g: 73,
        b: 58,
    },
    RGB8 {
        r: 72,
        g: 51,
        b: 38,
    },
    RGB8 {
        r: 49,
        g: 33,
        b: 23,
    },
    RGB8 {
        r: 31,
        g: 18,
        b: 12,
    },
    RGB8 {
        r: 255,
        g: 191,
        b: 146,
    },
    RGB8 {
        r: 208,
        g: 151,
        b: 113,
    },
    RGB8 {
        r: 167,
        g: 116,
        b: 84,
    },
    RGB8 {
        r: 131,
        g: 86,
        b: 60,
    },
    RGB8 {
        r: 99,
        g: 62,
        b: 39,
    },
    RGB8 {
        r: 72,
        g: 40,
        b: 23,
    },
    RGB8 {
        r: 49,
        g: 25,
        b: 12,
    },
    RGB8 { r: 31, g: 13, b: 5 },
];
