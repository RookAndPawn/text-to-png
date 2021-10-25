// This file is taken almost directly from svgtypes. I simplified the colors
// to exclude alpha values since this crate is only dealing with color not
// https://github.com/RazrFalcon/svgtypes/blob/master/src/colors.rs

use crate::Color;

pub(crate) static COLORS: Map<Color> = Map {
    key: 3213172566270843353,
    disps: &[
        (0, 37),
        (0, 74),
        (1, 0),
        (0, 22),
        (0, 92),
        (2, 125),
        (0, 73),
        (0, 8),
        (0, 2),
        (0, 4),
        (26, 16),
        (67, 121),
        (0, 3),
        (0, 4),
        (0, 60),
        (0, 39),
        (0, 130),
        (0, 15),
        (3, 38),
        (0, 136),
        (7, 75),
        (6, 141),
        (7, 67),
        (0, 14),
        (0, 128),
        (27, 111),
        (1, 31),
        (0, 12),
        (2, 49),
        (0, 2),
    ],
    entries: &[
        (
            "lightgrey",
            Color {
                r: 211,
                g: 211,
                b: 211,
            },
        ),
        (
            "lavenderblush",
            Color {
                r: 255,
                g: 240,
                b: 245,
            },
        ),
        (
            "deeppink",
            Color {
                r: 255,
                g: 20,
                b: 147,
            },
        ),
        (
            "seashell",
            Color {
                r: 255,
                g: 245,
                b: 238,
            },
        ),
        (
            "lightsalmon",
            Color {
                r: 255,
                g: 160,
                b: 122,
            },
        ),
        ("green", Color { r: 0, g: 128, b: 0 }),
        (
            "lightgreen",
            Color {
                r: 144,
                g: 238,
                b: 144,
            },
        ),
        ("black", Color { r: 0, g: 0, b: 0 }),
        (
            "deepskyblue",
            Color {
                r: 0,
                g: 191,
                b: 255,
            },
        ),
        (
            "mistyrose",
            Color {
                r: 255,
                g: 228,
                b: 225,
            },
        ),
        (
            "silver",
            Color {
                r: 192,
                g: 192,
                b: 192,
            },
        ),
        (
            "dimgray",
            Color {
                r: 105,
                g: 105,
                b: 105,
            },
        ),
        (
            "navajowhite",
            Color {
                r: 255,
                g: 222,
                b: 173,
            },
        ),
        (
            "royalblue",
            Color {
                r: 65,
                g: 105,
                b: 225,
            },
        ),
        (
            "peru",
            Color {
                r: 205,
                g: 133,
                b: 63,
            },
        ),
        (
            "darkgrey",
            Color {
                r: 169,
                g: 169,
                b: 169,
            },
        ),
        (
            "steelblue",
            Color {
                r: 70,
                g: 130,
                b: 180,
            },
        ),
        (
            "teal",
            Color {
                r: 0,
                g: 128,
                b: 128,
            },
        ),
        (
            "orangered",
            Color {
                r: 255,
                g: 69,
                b: 0,
            },
        ),
        (
            "mediumslateblue",
            Color {
                r: 123,
                g: 104,
                b: 238,
            },
        ),
        (
            "blueviolet",
            Color {
                r: 138,
                g: 43,
                b: 226,
            },
        ),
        (
            "cornflowerblue",
            Color {
                r: 100,
                g: 149,
                b: 237,
            },
        ),
        (
            "cyan",
            Color {
                r: 0,
                g: 255,
                b: 255,
            },
        ),
        (
            "beige",
            Color {
                r: 245,
                g: 245,
                b: 220,
            },
        ),
        (
            "goldenrod",
            Color {
                r: 218,
                g: 165,
                b: 32,
            },
        ),
        (
            "rosybrown",
            Color {
                r: 188,
                g: 143,
                b: 143,
            },
        ),
        (
            "yellow",
            Color {
                r: 255,
                g: 255,
                b: 0,
            },
        ),
        ("blue", Color { r: 0, g: 0, b: 255 }),
        ("darkblue", Color { r: 0, g: 0, b: 139 }),
        (
            "aliceblue",
            Color {
                r: 240,
                g: 248,
                b: 255,
            },
        ),
        (
            "white",
            Color {
                r: 255,
                g: 255,
                b: 255,
            },
        ),
        ("mediumblue", Color { r: 0, g: 0, b: 205 }),
        (
            "dodgerblue",
            Color {
                r: 30,
                g: 144,
                b: 255,
            },
        ),
        (
            "limegreen",
            Color {
                r: 50,
                g: 205,
                b: 50,
            },
        ),
        (
            "purple",
            Color {
                r: 128,
                g: 0,
                b: 128,
            },
        ),
        (
            "lightsteelblue",
            Color {
                r: 176,
                g: 196,
                b: 222,
            },
        ),
        (
            "lightslategray",
            Color {
                r: 119,
                g: 136,
                b: 153,
            },
        ),
        (
            "seagreen",
            Color {
                r: 46,
                g: 139,
                b: 87,
            },
        ),
        (
            "mediumvioletred",
            Color {
                r: 199,
                g: 21,
                b: 133,
            },
        ),
        (
            "slategrey",
            Color {
                r: 112,
                g: 128,
                b: 144,
            },
        ),
        (
            "darkslategrey",
            Color {
                r: 47,
                g: 79,
                b: 79,
            },
        ),
        (
            "turquoise",
            Color {
                r: 64,
                g: 224,
                b: 208,
            },
        ),
        (
            "paleturquoise",
            Color {
                r: 175,
                g: 238,
                b: 238,
            },
        ),
        (
            "lightgoldenrodyellow",
            Color {
                r: 250,
                g: 250,
                b: 210,
            },
        ),
        (
            "magenta",
            Color {
                r: 255,
                g: 0,
                b: 255,
            },
        ),
        (
            "darkseagreen",
            Color {
                r: 143,
                g: 188,
                b: 143,
            },
        ),
        (
            "lightcyan",
            Color {
                r: 224,
                g: 255,
                b: 255,
            },
        ),
        (
            "lightcoral",
            Color {
                r: 240,
                g: 128,
                b: 128,
            },
        ),
        (
            "mediumseagreen",
            Color {
                r: 60,
                g: 179,
                b: 113,
            },
        ),
        (
            "palegoldenrod",
            Color {
                r: 238,
                g: 232,
                b: 170,
            },
        ),
        (
            "palegreen",
            Color {
                r: 152,
                g: 251,
                b: 152,
            },
        ),
        (
            "darkslateblue",
            Color {
                r: 72,
                g: 61,
                b: 139,
            },
        ),
        (
            "moccasin",
            Color {
                r: 255,
                g: 228,
                b: 181,
            },
        ),
        (
            "forestgreen",
            Color {
                r: 34,
                g: 139,
                b: 34,
            },
        ),
        (
            "darkkhaki",
            Color {
                r: 189,
                g: 183,
                b: 107,
            },
        ),
        (
            "chartreuse",
            Color {
                r: 127,
                g: 255,
                b: 0,
            },
        ),
        (
            "floralwhite",
            Color {
                r: 255,
                g: 250,
                b: 240,
            },
        ),
        (
            "snow",
            Color {
                r: 255,
                g: 250,
                b: 250,
            },
        ),
        (
            "fuchsia",
            Color {
                r: 255,
                g: 0,
                b: 255,
            },
        ),
        (
            "orchid",
            Color {
                r: 218,
                g: 112,
                b: 214,
            },
        ),
        (
            "darkorchid",
            Color {
                r: 153,
                g: 50,
                b: 204,
            },
        ),
        ("darkred", Color { r: 139, g: 0, b: 0 }),
        (
            "darksalmon",
            Color {
                r: 233,
                g: 150,
                b: 122,
            },
        ),
        (
            "crimson",
            Color {
                r: 220,
                g: 20,
                b: 60,
            },
        ),
        ("lime", Color { r: 0, g: 255, b: 0 }),
        (
            "palevioletred",
            Color {
                r: 219,
                g: 112,
                b: 147,
            },
        ),
        (
            "lightseagreen",
            Color {
                r: 32,
                g: 178,
                b: 170,
            },
        ),
        (
            "ivory",
            Color {
                r: 255,
                g: 255,
                b: 240,
            },
        ),
        (
            "powderblue",
            Color {
                r: 176,
                g: 224,
                b: 230,
            },
        ),
        (
            "aquamarine",
            Color {
                r: 127,
                g: 255,
                b: 212,
            },
        ),
        (
            "darkturquoise",
            Color {
                r: 0,
                g: 206,
                b: 209,
            },
        ),
        (
            "lavender",
            Color {
                r: 230,
                g: 230,
                b: 250,
            },
        ),
        (
            "azure",
            Color {
                r: 240,
                g: 255,
                b: 255,
            },
        ),
        (
            "mediumturquoise",
            Color {
                r: 72,
                g: 209,
                b: 204,
            },
        ),
        (
            "lightgray",
            Color {
                r: 211,
                g: 211,
                b: 211,
            },
        ),
        ("transparent", Color { r: 0, g: 0, b: 0 }),
        (
            "gainsboro",
            Color {
                r: 220,
                g: 220,
                b: 220,
            },
        ),
        (
            "olivedrab",
            Color {
                r: 107,
                g: 142,
                b: 35,
            },
        ),
        (
            "papayawhip",
            Color {
                r: 255,
                g: 239,
                b: 213,
            },
        ),
        (
            "tomato",
            Color {
                r: 255,
                g: 99,
                b: 71,
            },
        ),
        (
            "midnightblue",
            Color {
                r: 25,
                g: 25,
                b: 112,
            },
        ),
        (
            "pink",
            Color {
                r: 255,
                g: 192,
                b: 203,
            },
        ),
        (
            "yellowgreen",
            Color {
                r: 154,
                g: 205,
                b: 50,
            },
        ),
        (
            "slategray",
            Color {
                r: 112,
                g: 128,
                b: 144,
            },
        ),
        ("red", Color { r: 255, g: 0, b: 0 }),
        (
            "indigo",
            Color {
                r: 75,
                g: 0,
                b: 130,
            },
        ),
        (
            "orange",
            Color {
                r: 255,
                g: 165,
                b: 0,
            },
        ),
        (
            "grey",
            Color {
                r: 128,
                g: 128,
                b: 128,
            },
        ),
        (
            "wheat",
            Color {
                r: 245,
                g: 222,
                b: 179,
            },
        ),
        (
            "darkgoldenrod",
            Color {
                r: 184,
                g: 134,
                b: 11,
            },
        ),
        (
            "lawngreen",
            Color {
                r: 124,
                g: 252,
                b: 0,
            },
        ),
        (
            "lightslategrey",
            Color {
                r: 119,
                g: 136,
                b: 153,
            },
        ),
        (
            "burlywood",
            Color {
                r: 222,
                g: 184,
                b: 135,
            },
        ),
        (
            "aqua",
            Color {
                r: 0,
                g: 255,
                b: 255,
            },
        ),
        (
            "saddlebrown",
            Color {
                r: 139,
                g: 69,
                b: 19,
            },
        ),
        (
            "oldlace",
            Color {
                r: 253,
                g: 245,
                b: 230,
            },
        ),
        (
            "lightskyblue",
            Color {
                r: 135,
                g: 206,
                b: 250,
            },
        ),
        (
            "violet",
            Color {
                r: 238,
                g: 130,
                b: 238,
            },
        ),
        (
            "dimgrey",
            Color {
                r: 105,
                g: 105,
                b: 105,
            },
        ),
        (
            "darkorange",
            Color {
                r: 255,
                g: 140,
                b: 0,
            },
        ),
        (
            "lightblue",
            Color {
                r: 173,
                g: 216,
                b: 230,
            },
        ),
        (
            "khaki",
            Color {
                r: 240,
                g: 230,
                b: 140,
            },
        ),
        (
            "coral",
            Color {
                r: 255,
                g: 127,
                b: 80,
            },
        ),
        (
            "brown",
            Color {
                r: 165,
                g: 42,
                b: 42,
            },
        ),
        (
            "mediumpurple",
            Color {
                r: 147,
                g: 112,
                b: 219,
            },
        ),
        (
            "linen",
            Color {
                r: 250,
                g: 240,
                b: 230,
            },
        ),
        (
            "mediumorchid",
            Color {
                r: 186,
                g: 85,
                b: 211,
            },
        ),
        (
            "indianred",
            Color {
                r: 205,
                g: 92,
                b: 92,
            },
        ),
        ("maroon", Color { r: 128, g: 0, b: 0 }),
        (
            "firebrick",
            Color {
                r: 178,
                g: 34,
                b: 34,
            },
        ),
        (
            "skyblue",
            Color {
                r: 135,
                g: 206,
                b: 235,
            },
        ),
        (
            "darkgray",
            Color {
                r: 169,
                g: 169,
                b: 169,
            },
        ),
        (
            "hotpink",
            Color {
                r: 255,
                g: 105,
                b: 180,
            },
        ),
        (
            "olive",
            Color {
                r: 128,
                g: 128,
                b: 0,
            },
        ),
        (
            "sienna",
            Color {
                r: 160,
                g: 82,
                b: 45,
            },
        ),
        (
            "cadetblue",
            Color {
                r: 95,
                g: 158,
                b: 160,
            },
        ),
        (
            "darkslategray",
            Color {
                r: 47,
                g: 79,
                b: 79,
            },
        ),
        (
            "slateblue",
            Color {
                r: 106,
                g: 90,
                b: 205,
            },
        ),
        (
            "plum",
            Color {
                r: 221,
                g: 160,
                b: 221,
            },
        ),
        (
            "mediumspringgreen",
            Color {
                r: 0,
                g: 250,
                b: 154,
            },
        ),
        (
            "thistle",
            Color {
                r: 216,
                g: 191,
                b: 216,
            },
        ),
        (
            "mintcream",
            Color {
                r: 245,
                g: 255,
                b: 250,
            },
        ),
        (
            "darkmagenta",
            Color {
                r: 139,
                g: 0,
                b: 139,
            },
        ),
        (
            "lemonchiffon",
            Color {
                r: 255,
                g: 250,
                b: 205,
            },
        ),
        (
            "bisque",
            Color {
                r: 255,
                g: 228,
                b: 196,
            },
        ),
        (
            "antiquewhite",
            Color {
                r: 250,
                g: 235,
                b: 215,
            },
        ),
        ("darkgreen", Color { r: 0, g: 100, b: 0 }),
        (
            "whitesmoke",
            Color {
                r: 245,
                g: 245,
                b: 245,
            },
        ),
        (
            "lightpink",
            Color {
                r: 255,
                g: 182,
                b: 193,
            },
        ),
        (
            "darkcyan",
            Color {
                r: 0,
                g: 139,
                b: 139,
            },
        ),
        (
            "tan",
            Color {
                r: 210,
                g: 180,
                b: 140,
            },
        ),
        (
            "blanchedalmond",
            Color {
                r: 255,
                g: 235,
                b: 205,
            },
        ),
        (
            "honeydew",
            Color {
                r: 240,
                g: 255,
                b: 240,
            },
        ),
        (
            "salmon",
            Color {
                r: 250,
                g: 128,
                b: 114,
            },
        ),
        (
            "lightyellow",
            Color {
                r: 255,
                g: 255,
                b: 224,
            },
        ),
        (
            "springgreen",
            Color {
                r: 0,
                g: 255,
                b: 127,
            },
        ),
        (
            "cornsilk",
            Color {
                r: 255,
                g: 248,
                b: 220,
            },
        ),
        (
            "sandybrown",
            Color {
                r: 244,
                g: 164,
                b: 96,
            },
        ),
        (
            "mediumaquamarine",
            Color {
                r: 102,
                g: 205,
                b: 170,
            },
        ),
        (
            "darkviolet",
            Color {
                r: 148,
                g: 0,
                b: 211,
            },
        ),
        (
            "darkolivegreen",
            Color {
                r: 85,
                g: 107,
                b: 47,
            },
        ),
        (
            "gold",
            Color {
                r: 255,
                g: 215,
                b: 0,
            },
        ),
        (
            "peachpuff",
            Color {
                r: 255,
                g: 218,
                b: 185,
            },
        ),
        (
            "greenyellow",
            Color {
                r: 173,
                g: 255,
                b: 47,
            },
        ),
        (
            "gray",
            Color {
                r: 128,
                g: 128,
                b: 128,
            },
        ),
        ("navy", Color { r: 0, g: 0, b: 128 }),
        (
            "ghostwhite",
            Color {
                r: 248,
                g: 248,
                b: 255,
            },
        ),
        (
            "chocolate",
            Color {
                r: 210,
                g: 105,
                b: 30,
            },
        ),
    ],
};

pub(crate) fn from_str(text: &str) -> Option<Color> {
    COLORS.get(text).cloned()
}

// A stripped down `phf` crate fork.
//
// https://github.com/sfackler/rust-phf

use std::borrow::Borrow;
use std::hash::Hasher;

pub struct Map<V: 'static> {
    pub key: u64,
    pub disps: &'static [(u32, u32)],
    pub entries: &'static [(&'static str, V)],
}

impl<V> Map<V> {
    pub fn get(&self, key: &str) -> Option<&V> {
        let hash = hash(key, self.key);
        let index = get_index(hash, &*self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        let b = entry.0.borrow();
        if b == key {
            Some(&entry.1)
        } else {
            None
        }
    }
}

#[inline]
fn hash(x: &str, key: u64) -> u64 {
    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, key);
    hasher.write(x.as_bytes());
    hasher.finish()
}

#[inline]
fn get_index(hash: u64, disps: &[(u32, u32)], len: usize) -> u32 {
    let (g, f1, f2) = split(hash);
    let (d1, d2) = disps[(g % (disps.len() as u32)) as usize];
    displace(f1, f2, d1, d2) % (len as u32)
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1;

    (
        (hash & MASK) as u32,
        ((hash >> BITS) & MASK) as u32,
        ((hash >> (2 * BITS)) & MASK) as u32,
    )
}

#[inline]
fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}
