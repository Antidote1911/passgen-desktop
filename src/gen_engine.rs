pub mod gen_engine {
    use crate::generator::generator::Generator;
    use rand::Rng;

    const LETTERS_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const U_LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS_CHARSET: &[u8] = b"0123456789";
    const SPEC_SYMB_CHARSET: &[u8] = b")([]{}*&^%$#@!~";
    const LET_NUM_DRC_FREE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz\
                            23456789"; // double readable characters free / без двоякочитаемых символов

    impl Generator {
        pub fn generate_pass(&self) -> String {
            let mut rng = rand::thread_rng();
            let mut pass_assembly: Vec<&[u8]> = Vec::new();
            //let mut pass_charset: Vec<u8> = Vec::new();

            //let mut pass_charset: & [u8] = LET_NUM_DRC_FREE;

            if self.convenience_criterion {
                pass_assembly.push(LET_NUM_DRC_FREE);
            } else {
                if self.letters {
                    pass_assembly.push(LETTERS_CHARSET);
                }
                if self.u_letters {
                    pass_assembly.push(U_LETTERS_CHARSET);
                }
                if self.numbs {
                    pass_assembly.push(NUMBERS_CHARSET);
                }
            }
            if self.spec_symbs {
                pass_assembly.push(SPEC_SYMB_CHARSET);
            }

            let pass_charset: Vec<u8> = pass_assembly.into_iter().flatten().cloned().collect();
            (0..self.pwd_len)
                .map(|_| pass_charset[rng.gen_range(0..pass_charset.len())] as char)
                .collect()
        }

        pub fn is_valid_pwd_by_consist(&self, pass: String) -> bool {
            let pwd_in_bytes = pass.clone().into_bytes();

            let check_to_available_for = |symbols: &[u8]| -> bool {
                let mut res = false;
                for ch in &pwd_in_bytes {
                    if symbols.contains(&ch) {
                        res = true;
                        break;
                    }
                }
                res
            };

            if self.letters || self.convenience_criterion {
                if !check_to_available_for(LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.u_letters || self.convenience_criterion {
                if !check_to_available_for(U_LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.numbs || self.convenience_criterion {
                if !check_to_available_for(NUMBERS_CHARSET) {
                    return false;
                }
            }
            if self.spec_symbs {
                if !check_to_available_for(SPEC_SYMB_CHARSET) {
                    return false;
                }
            }
            true
        }
    }
}
