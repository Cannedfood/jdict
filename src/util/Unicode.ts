export function is_cjk_base(codepoint: number) { return codepoint >= 0x4e00 && codepoint <= 0x9FFF; }
export function is_cjk_extension_a(codepoint: number) { return codepoint >= 0x3400  && codepoint <= 0x4DBF;  }
export function is_cjk_extension_b(codepoint: number) { return codepoint >= 0x20000 && codepoint <= 0x2A6DF; }
export function is_cjk_extension_c(codepoint: number) { return codepoint >= 0x2A700 && codepoint <= 0x2B73F; }
export function is_cjk_extension_d(codepoint: number) { return codepoint >= 0x2B740 && codepoint <= 0x2B81F; }
export function is_cjk_extension_e(codepoint: number) { return codepoint >= 0x2B820 && codepoint <= 0x2CEAF; }
export function is_cjk_extension_f(codepoint: number) { return codepoint >= 0x2CEB0 && codepoint <= 0x2EBEF; }
export function is_cjk_extension_g(codepoint: number) { return codepoint >= 0x30000 && codepoint <= 0x3134F; }
export function is_cjk_compat     (codepoint: number) { return codepoint >= 0xF900  && codepoint <= 0xFAFF; }
export function is_cjk(codepoint: number) {
	return (
		is_cjk_base(codepoint) ||
		is_cjk_extension_a(codepoint) ||
		is_cjk_extension_b(codepoint) ||
		is_cjk_extension_c(codepoint) ||
		is_cjk_extension_d(codepoint) ||
		is_cjk_extension_e(codepoint) ||
		is_cjk_extension_f(codepoint) ||
		is_cjk_extension_g(codepoint) ||
		is_cjk_compat(codepoint)
	);
}
