pub struct Verb {
    dictionary_form: String,
    kana: String,
    verb_type: VerbType,
}

pub enum VerbType {
    U,
    Ru,
    Irregular,
}

impl Verb {
    pub fn new(dictionary_form: &str, kana: &str, verb_type: VerbType) -> Self {
	Self {
	    dictionary_form: dictionary_form.to_string(),
	    kana: kana.to_string(),
	    verb_type,
	}
    }
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
    pub fn debug(&self) {
	println!("dictionary: {}", &self.dictionary_form);
	println!("kana: {}", &self.kana);
	println!("stem: {}", &self.stem());
	println!("te-form: {}", &self.te_form());
    }
}

pub fn run_verb_test() {
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
