use super::spinlock::SpinLock;

pub struct SleepLock<'a> {
    locked: bool,
    lk: SpinLock<'a>,

    name: &'a str,
    pid: u32,
}
