// The search should handle:
// - Often misheard syllables (e.g. resoko instead of reisouko)
// - Typical beginner errors (e.g. steru instead of suteru)
// - Conjugations
//   - -masu -> -ru

use bitmask_enum::bitmask;

#[bitmask]
#[derive(Default)]
pub enum RuleFlags {
	VerbsOnly,
	NounsOnly,
}

pub struct Rule {
	pub pattern: &'static str,
	pub replace: &'static str,
	pub cost: u32,
	pub description: &'static str,
	pub flags: RuleFlags,
}
impl Rule {
	pub fn apply(rule: Rule, word: &str, word_flags: RuleFlags) -> Option<String> {
		if !word_flags.contains(rule.flags) {
			return None;
		}

		if let Some(ending) = rule.pattern.strip_prefix('*') {
			if let Some(start) = word.strip_suffix(ending) {
				return Some(concat(&[ start, rule.replace ]));
			}
		}
		else if let Some(start) = rule.pattern.strip_suffix('*') {
			if let Some(ending) = word.strip_prefix(start) {
				return Some(concat(&[ rule.replace, ending ]));
			}
		}
		else if let Some(at) = word.find(rule.pattern) {
			return Some(concat(&[ &word[..at], rule.replace, &word[at + rule.pattern.len()..] ]));
		}
		None
	}
}

const fn rule(
	pattern: &'static str,
	replace: &'static str,
	cost: u32,
	description: &'static str,
	flags: RuleFlags,
) -> Rule {
	Rule { pattern, replace, cost, description, flags }
}

const fn misspelling(
	pattern: &'static str,
	replace: &'static str,
	cost: u32,
	description: &'static str,
) -> Rule {
	rule(pattern, replace, cost, description, RuleFlags::none())
}

pub const RULES: &[Rule] = &[
	//Misspellings\\____________________________

	// Misspellings (common)
	misspelling("s", "z", 0, "'z' sounds like 's'"),
	misspelling("o", "ou", 0, "'ou' sounds like 'o'"),
	misspelling("e", "ei", 0, "'ei' sounds like 'e'"),

	// Misspellings (less common)
	misspelling("a", "aa", 1, "'aa' sounds like 'a'"),
	misspelling("e", "ee", 1, "'ee' sounds like 'e'"),
	misspelling("i", "ii", 1, "'ii' sounds like 'i'"),
	misspelling("o", "oo", 1, "'oo' sounds like 'o'"),
	misspelling("u", "uu", 1, "'uu' sounds like 'u'"),

	// Misspellings (beginner errors)
	misspelling("st", "sut", 2, "silent 'u' in 'su'"),
	misspelling("sp", "sup", 2, "silent 'u' in 'su'"),
	misspelling("sk", "suk", 2, "silent 'u' in 'su'"),
	misspelling("sh", "shi", 2, "silent 'i' in 'shi'"),

	// Misspellings (rare beginner errors)
	misspelling("t", "tt", 3, "emphasized 't'"),
	misspelling("k", "kk", 3, "emphasized 'k'"),
	misspelling("p", "pp", 3, "emphasized 'p'"),

	//Deconjugations\\____________________________

	// Honorifics
	rule("go*", "", 2, "Removed Go- honorific", RuleFlags::NounsOnly),
	rule("o*",  "", 2, "Removed O- honorific", RuleFlags::NounsOnly),

	// Nouns

	// Verbs
	rule("*nai",     "ru", 1, "Deconjugation: -nai negative", RuleFlags::VerbsOnly),
	rule("*tai",     "ru", 1, "Deconjugation: -tai TODO", RuleFlags::VerbsOnly),
	rule("*masu",    "ru", 1, "Deconjugation: -masu stem", RuleFlags::VerbsOnly),
	rule("*masen",   "ru", 1, "Deconjugation: -masen negative", RuleFlags::VerbsOnly),
	rule("*mashita", "ru", 1, "Deconjugation: -mashita simple past", RuleFlags::VerbsOnly),
];

#[inline]
fn concat(strns: &[&str]) -> String {
	let mut result = String::with_capacity(strns.iter().map(|s| s.len()).sum());
	for s in strns {
		result.push_str(s);
	}
	result
}
