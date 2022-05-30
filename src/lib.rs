mod mines;
mod random;

use mines::*;

use ::wasm_bindgen::prelude::*;

use std::cell::RefCell;

thread_local! {
    static FIELD: RefCell<MinesGame> = RefCell::new(new_field());
}

fn new_field() -> MinesGame {
    random::PRNG.with(|prng| MinesGame::new(9, 9, 8, &mut prng.borrow_mut()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_state() -> String {
    FIELD.with(|field| field.borrow().to_string())
}

#[wasm_bindgen]
pub fn open_field(x: usize, y: usize) {
    FIELD.with(|field| field.borrow_mut().click_open((x, y)));
}

#[wasm_bindgen]
pub fn toggle_flag(x: usize, y: usize) {
    FIELD.with(|field| field.borrow_mut().toggle_flag((x, y)));
}

#[cfg(test)]
mod tests {
    use super::*;

    use prng::Prng16;

    #[test]
    fn make_a_random_field() {
        let mut prng = Prng16::new(random::get_prng_seed());

        let field = MinesGame::new(9, 9, 10, &mut prng);

        println!("{}", field);
    }

    #[test]
    fn open() {
        let mut prng = Prng16::new(random::get_prng_seed());

        let mut field = MinesGame::new(9, 9, 10, &mut prng);

        field.open((4, 4));

        println!("{}", field);
        panic!();
    }

    #[test]
    fn flag() {
        let mut prng = Prng16::new(random::get_prng_seed());

        let mut field = MinesGame::new(9, 9, 10, &mut prng);

        field.toggle_flag((6, 4));
        field.toggle_flag((4, 6));
        field.toggle_flag((2, 4));
        field.toggle_flag((4, 2));
        field.open((4, 4));

        println!("{}", field);
        panic!();
    }
}
