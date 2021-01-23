//! Host Controller Runtime Registers.

use core::fmt;

use crate::error::Error;

/// Event Ring Segment Table Size Register.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EventRingSegmentTableSizeRegister(u32);
impl EventRingSegmentTableSizeRegister {
    /// Sets the number of segments the Event Ring Segment Table supports.
    pub fn set(&mut self, s: u16) {
        self.0 = s.into();
    }
}

/// Event Ring Segment Table Base Address Register.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EventRingSegmentTableBaseAddressRegister(u64);
impl EventRingSegmentTableBaseAddressRegister {
    /// Sets the address of the Event Ring Segment Table. It must be 64 byte aligned.
    ///
    /// # Errors
    ///
    /// This method may return an [`Error::NotAligned`] error if the address is not 64 byte aligned.
    pub fn set(&mut self, a: u64) -> Result<(), Error> {
        if a.trailing_zeros() >= 6 {
            self.0 = a;
            Ok(())
        } else {
            Err(Error::NotAligned {
                alignment: 64,
                address: a,
            })
        }
    }
}

/// Event Ring Dequeue Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingDequeuePointerRegister(u64);
impl EventRingDequeuePointerRegister {
    /// Returns the address of the current Event Ring Dequeue Pointer.
    #[must_use]
    pub fn event_ring_dequeue_pointer(self) -> u64 {
        self.0 & 0b1111
    }

    /// Sets the address of the current Event Ring Dequeue Pointer. It must be 16 byte aligned.
    ///
    /// # Errors
    ///
    /// This method may return an [`Error::NotAligned`] error if the address is not 16 byte aligned.
    pub fn set_event_ring_dequeue_pointer(&mut self, p: u64) -> Result<(), Error> {
        if p.trailing_zeros() >= 4 {
            self.0 = p;
            Ok(())
        } else {
            Err(Error::NotAligned {
                alignment: 16,
                address: p,
            })
        }
    }
}
impl fmt::Debug for EventRingDequeuePointerRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventRingDequeuePointerRegister")
            .field(
                "event_ring_dequeue_pointer",
                &self.event_ring_dequeue_pointer(),
            )
            .finish()
    }
}
