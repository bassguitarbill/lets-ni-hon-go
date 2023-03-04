/*fn main() {
    let verbs = vec![
        Verb::new("taberu", VerbType::Ru),
        Verb::new("arau", VerbType::U),
        Verb::new("matsu", VerbType::U),
        Verb::new("toru", VerbType::U),
        Verb::new("yomu", VerbType::U),
        Verb::new("asobu", VerbType::U),
        Verb::new("shinu", VerbType::U),
        Verb::new("kaku", VerbType::U),
        Verb::new("isogu", VerbType::U),
        Verb::new("hanasu", VerbType::U),
    ];
    verbs
        .iter()
        .for_each(|v| println!("{}: {}", v.dictionary_form, v.te_form()));
}

enum Word {
    Adjective(Adjective),
    Verb(Verb),
    Noun(Noun),
}

struct Verb {
    dictionary_form: String,
    verb_type: VerbType,
}

impl Verb {
    fn new(s: &str, verb_type: VerbType) -> Self {
        Self {
            dictionary_form: s.to_string(),
            verb_type,
        }
    }
}

trait Stem {
    fn stem(&self) -> String;
}

impl Stem for Verb {
    fn stem(&self) -> String {
        let last_three = &self.dictionary_form[self.dictionary_form.len() - 3..];
        let last_two = &self.dictionary_form[self.dictionary_form.len() - 2..];
        match last_three {
            "tsu" => (&self.dictionary_form[..self.dictionary_form.len() - 3]).to_string(),
            _ => match last_two {
                "ku" | "su" | "nu" | "mu" | "ru" | "bu" | "gu" => {
                    (&self.dictionary_form[..self.dictionary_form.len() - 2]).to_string()
                }
                _ => (&self.dictionary_form[..self.dictionary_form.len() - 1]).to_string(),
            },
        }
    }
}

trait TeForm {
    fn te_form(&self) -> String;
}

impl TeForm for Verb {
    fn te_form(&self) -> String {
        match self.verb_type {
            VerbType::Ru => {
                format!("{}{}", &self.stem(), "te")
            }
            VerbType::U => {
                let last_three = &self.dictionary_form[self.dictionary_form.len() - 3..];
                let last_two = &self.dictionary_form[self.dictionary_form.len() - 2..];
                match last_three {
                    "tsu" => format!("{}{}", &self.stem(), "tte"),
                    _ => match last_two {
                        "ru" => {
                            format!("{}{}", &self.stem(), "tte")
                        }
                        "mu" | "bu" | "nu" => {
                            format!("{}{}", &self.stem(), "nde")
                        }
                        "ku" => {
                            format!("{}{}", &self.stem(), "ite")
                        }
                        "gu" => {
                            format!("{}{}", &self.stem(), "ide")
                        }
                        "su" => {
                            format!("{}{}", &self.stem(), "shite")
                        }
                        _ => {
                            format!("{}{}", &self.stem(), "tte")
                        }
                    },
                }
            }
            VerbType::Irregular => "".to_string(),
        }
    }
}

enum VerbType {
    U,
    Ru,
    Irregular,
}

struct Noun {
    dictionary_form: String,
}

struct Adjective {
    dictionary_form: String,
    adjective_type: AdjectiveType,
}

enum AdjectiveType {
    NaAdjective,
    IAdjective,
}
*/

struct Verb {
    dictionary_form: String,
    kana: String,
    verb_type: VerbType,
}

impl Verb {
    fn new(dictionary_form: &str, kana: &str, verb_type: VerbType) -> Self {
	Self {
	    dictionary_form: dictionary_form.to_string(),
	    kana: kana.to_string(),
	    verb_type,
	}
    }
}

enum VerbType {
    U,
    Ru,
    Irregular,
}

impl Verb {
    fn stem(&self) -> String {
        let length = self.dictionary_form.chars().count();
        self.dictionary_form.chars().take(length - 1).collect()
    }
    fn te_form(&self) -> String {
	match &self.verb_type {
	    VerbType::Irregular => {
		match &(self.dictionary_form)[..] {
		    "する" => { "して".to_string() },
		    "来る" => { "来て".to_string() },
		    _ => {
			let length = self.dictionary_form.chars().count();
			let without_suru: String = self.dictionary_form.chars().take(length - 2).collect();
			format!("{}{}", without_suru, "して")
		    }
		}
	    },
	    VerbType::Ru => { format!("{}{}", &self.stem(), "て") },
	    VerbType::U => {
		let last_character = &self.dictionary_form.chars().last().unwrap();
		let replacement = match last_character {
		    'む'|'ぶ'|'ぬ' => { "んで" },
		    'く' => {
			match &(self.dictionary_form)[..] {
			    "行く" => { "って" },
			    _ => { "いて" },
			}
		    },
		    'ぐ' => { "いで" },
		    'す' => { "して" },
		    _ => { "って" },
		};
		format!("{}{}", self.stem(), replacement)
	    },
	}
    }
    fn debug(&self) {
	println!("dictionary: {}", &self.dictionary_form);
	println!("kana: {}", &self.kana);
	println!("stem: {}", &self.stem());
	println!("te-form: {}", &self.te_form());
    }
}

fn main() {
    let verbs = vec![
	Verb::new("食べる","たべる",VerbType::Ru),
	Verb::new("合う","あう",VerbType::U),
	Verb::new("働く","はたらく",VerbType::U),
	Verb::new("行く","いく",VerbType::U),
	Verb::new("急ぐ","いそぐ",VerbType::U),
	Verb::new("話す","はなく",VerbType::U),
	Verb::new("持つ","もつ",VerbType::U),
	Verb::new("死ぬ","しぬ",VerbType::U),
	Verb::new("遊ぶ","あそぶ",VerbType::U),
	Verb::new("飲む","のむ",VerbType::U),
	Verb::new("見る","みる",VerbType::U),
	Verb::new("する","する",VerbType::Irregular),
	Verb::new("来る","来る",VerbType::Irregular),
	Verb::new("勉強する","べんきょうする",VerbType::Irregular),
    ];

    verbs.iter().for_each(|v| { v.debug(); println!(""); } );
}
