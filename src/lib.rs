// pub use self::subprocess::*;
pub mod subprocess {
    use nix::unistd::{execvp, fork, ForkResult};
    use std::ffi::CStr;
    pub struct SubProcess {
        name: String,
    }
    impl SubProcess {
        pub fn new(name: &str) -> Result<SubProcess, &'static str> {
            Ok(SubProcess { name: name.into() })
        }

        pub fn run(self) {
            match fork() {
                Ok(ForkResult::Parent { .. }) => {
                    eprintln!("parent");
                }
                Ok(ForkResult::Child) => {
                    eprintln!("about to exec");
                    let _exec_res = execvp(
                        &CStr::from_bytes_with_nul(&[self.name.as_bytes(), b"\0"].concat())
                            .expect("Invalid CStr"),
                        &[],
                    );
                }
                Err(nix_err) => eprintln!("{}", nix_err.to_string()),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::subprocess::*;
    #[test]
    fn init() {
        assert!(SubProcess::new("test").is_ok(), true);
    }
    #[test]
    fn run() {
        let sp = SubProcess::new("./fsystem").unwrap();
        sp.run();
    }
}
