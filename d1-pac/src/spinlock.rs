#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Spinlock System Status Register"]
    pub spinlock_systatus: SPINLOCK_SYSTATUS,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Spinlock Status Register"]
    pub spinlock_status: SPINLOCK_STATUS,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Spinlock Interrupt Enable Register"]
    pub spinlock_irq_en: SPINLOCK_IRQ_EN,
    _reserved3: [u8; 0x1c],
    #[doc = "0x40 - Spinlock Interrupt Status Register"]
    pub spinlock_irq_sta: SPINLOCK_IRQ_STA,
    _reserved4: [u8; 0x3c],
    #[doc = "0x80..0x94 - Spinlock Lockid Register"]
    pub spinlock_lockid: [SPINLOCK_LOCKID; 5],
    _reserved5: [u8; 0x6c],
    #[doc = "0x100..0x180 - Spinlock Register"]
    pub spinlock_lock: [SPINLOCK_LOCK; 32],
}
#[doc = "spinlock_systatus (r) register accessor: an alias for `Reg<SPINLOCK_SYSTATUS_SPEC>`"]
pub type SPINLOCK_SYSTATUS = crate::Reg<spinlock_systatus::SPINLOCK_SYSTATUS_SPEC>;
#[doc = "Spinlock System Status Register"]
pub mod spinlock_systatus;
#[doc = "spinlock_status (r) register accessor: an alias for `Reg<SPINLOCK_STATUS_SPEC>`"]
pub type SPINLOCK_STATUS = crate::Reg<spinlock_status::SPINLOCK_STATUS_SPEC>;
#[doc = "Spinlock Status Register"]
pub mod spinlock_status;
#[doc = "spinlock_irq_en (rw) register accessor: an alias for `Reg<SPINLOCK_IRQ_EN_SPEC>`"]
pub type SPINLOCK_IRQ_EN = crate::Reg<spinlock_irq_en::SPINLOCK_IRQ_EN_SPEC>;
#[doc = "Spinlock Interrupt Enable Register"]
pub mod spinlock_irq_en;
#[doc = "spinlock_irq_sta (rw) register accessor: an alias for `Reg<SPINLOCK_IRQ_STA_SPEC>`"]
pub type SPINLOCK_IRQ_STA = crate::Reg<spinlock_irq_sta::SPINLOCK_IRQ_STA_SPEC>;
#[doc = "Spinlock Interrupt Status Register"]
pub mod spinlock_irq_sta;
#[doc = "spinlock_lockid (r) register accessor: an alias for `Reg<SPINLOCK_LOCKID_SPEC>`"]
pub type SPINLOCK_LOCKID = crate::Reg<spinlock_lockid::SPINLOCK_LOCKID_SPEC>;
#[doc = "Spinlock Lockid Register"]
pub mod spinlock_lockid;
#[doc = "spinlock_lock (rw) register accessor: an alias for `Reg<SPINLOCK_LOCK_SPEC>`"]
pub type SPINLOCK_LOCK = crate::Reg<spinlock_lock::SPINLOCK_LOCK_SPEC>;
#[doc = "Spinlock Register"]
pub mod spinlock_lock;
