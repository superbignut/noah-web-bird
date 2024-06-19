use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn whose_that_dog()->String {
    "Mister Dog".into()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
