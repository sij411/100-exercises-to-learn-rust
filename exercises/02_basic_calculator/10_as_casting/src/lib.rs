// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let x: u32 = 47; // bcz u32 is bigger than u16
        assert_eq!(47u16 as u32, x);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u8_to_i8() {
            // The compiler is smart enough to know that the value 255 cannot fit
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // this guardrail to make this (bad) conversion possible.
        // The compiler is only able to pick on this because the value is a
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // catch this at compile time.
        let y = 255 as i8;
        assert_eq!(255 as i8, y);
    }

    #[test]
    fn bool_to_u8() {
        let z: u8 = 1;
        assert_eq!(true as u8, z);
    }
}
