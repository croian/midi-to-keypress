// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/digraphs.html

pub const KEYMAP: [&str; 128] = A;

const A: [&str; 128] = [

    // 0 - 11
    "", "", "", "",
    "", "", "", "",
    "", "", "", "",

    // 12 - 23
    "F1", "F2", "F3", "F4",
    "F5", "F6", "F7", "F8",
    "F9", "F10", "F11", "F12",

    // 24 - 35
    "(", // todo: shift + 9
    "[",
    ")", // todo: shift + 0
    "]",
    "Control",

    "Alt",
    "Escape",
    "Shift",
    "Tab",
    "a",
    "Space",
    "b",

    // 36 - 47
    "c", ",", "d", "s", "e",
    "f", "t", "g", "u", "h", "v", "i",

    // 48 - 59
    "j", "w", "k", "x", "l",     "m", "y", "n", "z", "o", ".", "p",

    // 60 - 71
    "q",
    "Return",
    "r",
    "Backspace",
    "Shift",

    "Alt",
    ";",
    "Control",
    "'",
    "-",
    "`",
    "=",

    // 72 - 83
    "",
    "/",
    "",
    "\\",
    "LeftArrow",

    "DownArrow",
    "UpArrow",
    "RightArrow",
    "Home",
    "End",
    "PageUp",
    "PageDown",

    // 84 - 95
    "1", "2", "3", "4", "5",     "6", "7", "8", "9", "0", "", "",

    // 96 - 107
    "", "", "", "", "",     "", "", "", "", "", "", "",

    // 108 - 119
    "", "", "", "", "",     "", "", "", "", "", "", "",

    // 120 - 127
    "", "", "", "", "",     "", "", "",
];
