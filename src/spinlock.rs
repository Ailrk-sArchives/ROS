use super::proc;

pub struct SpinLock<'a> {
    pub locked: bool,
    pub name: &'a str,
    pub cpu: &'a proc::Cpu<'a>,
}

pub impl SpinLock<'a> {

    pub fn new<'b>() -> SpinLock<'b> {

    }
}
