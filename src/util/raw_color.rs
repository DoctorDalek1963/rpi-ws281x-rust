pub type RawColor = [u8; 4];

#[cfg(test)]
mod tests {
    use super::super::super::bindings::ws2811_led_t;
    use super::RawColor;
    use std::mem;
    #[test]
    fn test_size_compatible() {
        assert_eq!(mem::size_of::<ws2811_led_t>(), mem::size_of::<RawColor>());
    }
}
