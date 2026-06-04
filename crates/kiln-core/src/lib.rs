#![forbid(unsafe_code)]

pub const KILN_VERSION: &str = "v0";

#[cfg(test)]
mod tests {
    use super::KILN_VERSION;

    #[test]
    fn exposes_v0_foundation_version() {
        assert_eq!(KILN_VERSION, "v0");
    }
}
