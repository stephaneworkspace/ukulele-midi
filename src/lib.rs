#[macro_use]
extern crate log;

pub mod ghakuf_customize;
pub mod hodge;
pub mod synthrs_customize;
pub mod ukulele;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
