const MAN: u32 = 10000;
const SEN: u32 = 1000;
const HYAKU: u32 = 100;
const JYUU: u32 = 10;

const DIGITS_KANJI: &'static [&'static str] = &[
    "",
    "一",
    "二",
    "三",
    "四",
    "五",
    "六",
    "七",
    "八",
    "九",
];

const DIGITS_KANA: &'static [&'static str] = &[
    "",
    "いち",
    "に",
    "さん",
    "よん",
    "ご",
    "ろく",
    "なな",
    "はち",
    "きゅう",
];

const MAN_KANA: &'static [&'static str] = &[
    "",
    "いちまん",
    "にまん",
    "さんまん",
    "よんまん",
    "ごまん",
    "ろくまん",
    "ななまん",
    "はちまん",
    "きゅうまん",
];

const SEN_KANA: &'static [&'static str] = &[
    "",
    "せん",
    "にせん",
    "さんぜん",
    "よんせん",
    "ごせん",
    "ろくせん",
    "ななせん",
    "はっせん",
    "きゅうせん",
];

const HYAKU_KANA: &'static [&'static str] = &[
    "",
    "ひゃく",
    "にひゃく",
    "さんびゃく",
    "よんひゃく",
    "ごひゃく",
    "ろっぴゃく",
    "ななひゃく",
    "はっぴゃく",
    "きゅうひゃく",
];

trait ToJapanese {
    fn to_kanji(&self) -> String;
    fn to_kana(&self) -> String;
    fn _to_kanji(&self) -> String;
    fn _to_kana(&self) -> String;
}

impl ToJapanese for u32 {
    fn to_kana(&self) -> String {
	match self {
	    0 =>  { "ゼロ".to_string() },
	    _ => self._to_kana()
	}
    }

    fn to_kanji(&self) -> String {
	match self {
	    0 =>  { "ゼロ".to_string() },
	    _ => self._to_kanji()
	}
    }
		
    fn _to_kana(&self) -> String {
	let mut number = *self;
	if number > 99999999 {
	    unimplemented!("Don't be silly, numbers can't get that high");
	}
	let man = number / MAN;
	number = number % MAN;

	if man > 0 {
	    format!("{}{} {}", man.to_kana(), "まん", number.to_kana())
	} else {
	    let sen: usize = (number / SEN) as usize;
	    number = number % SEN;
	    if sen > 0 {
		format!("{} {}", SEN_KANA[sen], number.to_kana())
	    } else {
		let hyaku: usize = (number / HYAKU) as usize;
		number = number % HYAKU;
		if hyaku > 0 {
		    format!("{} {}", HYAKU_KANA[hyaku], number.to_kana())
		} else {
		    let jyuu = number / JYUU;
		    number = number % JYUU;
		    if jyuu > 1 {
			format!("{}{} {}", jyuu.to_kana(), "じゅう", number.to_kanji())
		    } else if jyuu == 1 {
			format!("{} {}", "じゅう", number.to_kana())
		    } else {
			DIGITS_KANA[number as usize].to_string()
		    }
		}
	    }
	}
    }

    fn _to_kanji(&self) -> String {
	let mut number = *self;
	if number > 99999999 {
	    unimplemented!("Don't be silly, numbers can't get that high");
	}
	let man = number / MAN;
	number = number % MAN;

	if man > 0 {
	    format!("{}{} {}", man.to_kanji(), "万", number.to_kanji())
	} else {
	    let sen = number / SEN;
	    number = number % SEN;
	    if sen > 1 {
		format!("{}{} {}", sen.to_kanji(), "千", number.to_kanji())
	    } else if sen == 1 {
		format!("{} {}", "千", number.to_kanji())
	    } else {
		let hyaku = number / HYAKU;
		number = number % HYAKU;
		if hyaku > 1 {
		    format!("{}{} {}", hyaku.to_kanji(), "百", number.to_kanji())
		} else if hyaku == 1 {
		    format!("{} {}", "百", number.to_kanji())
		} else {
		    let jyuu = number / JYUU;
		    number = number % JYUU;
		    if jyuu > 1 {
			format!("{}{} {}", jyuu.to_kanji(), "十", number.to_kanji())
		    } else if jyuu == 1 {
			format!("{} {}", "十", number.to_kanji())
		    } else {
			DIGITS_KANJI[number as usize].to_string()
		    }
		}
	    }
	}
    }
}

pub fn numbers_test() {
    let n = 0;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 1;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 5;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 10;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 100;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 1000;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 10000;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 100000;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 1000000;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
    let n = 99999999;
    println!("{}: {} - {}", n, n.to_kana(), n.to_kanji());
}
