use core::sync::atomic::{AtomicU32, Ordering};

pub(crate) struct AtomicU64 {
    lo: AtomicU32,
    hi: AtomicU32,
}

impl AtomicU64 {
    pub(crate) const fn new(v: u64) -> Self {
        Self {
            lo: AtomicU32::new(v as u32),
            hi: AtomicU32::new((v >> 32) as u32),
        }
    }

    pub(crate) fn load(&self, order: Ordering) -> u64 {
        let lo = self.lo.load(order) as u64;
        let hi = self.hi.load(order) as u64;
        (hi << 32) | lo
    }

    pub(crate) fn store(&self, v: u64, order: Ordering) {
        self.lo.store(v as u32, order);
        self.hi.store((v >> 32) as u32, order);
    }

    pub(crate) fn fetch_or(&self, v: u64, order: Ordering) -> u64 {
        let lo = self.lo.fetch_or(v as u32, order) as u64;
        let hi = self.hi.fetch_or((v >> 32) as u32, order) as u64;
        (hi << 32) | lo
    }

    pub(crate) fn fetch_and(&self, v: u64, order: Ordering) -> u64 {
        let lo = self.lo.fetch_and(v as u32, order) as u64;
        let hi = self.hi.fetch_and((v >> 32) as u32, order) as u64;
        (hi << 32) | lo
    }
}

impl core::fmt::Debug for AtomicU64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AtomicU64({})", self.load(core::sync::atomic::Ordering::Relaxed))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AtomicU64 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u64(self.load(core::sync::atomic::Ordering::Relaxed))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AtomicU64 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        u64::deserialize(d).map(AtomicU64::new)
    }
}
