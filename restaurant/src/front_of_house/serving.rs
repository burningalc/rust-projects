fn take_order() {}

fn serve_order() {
    self::back_of_house::cook_order();
}

fn take_payment() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    pub fn cook_order() {}
}
