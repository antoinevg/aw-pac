#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_MEMORY_POINTER_PARTITION {
    #[doc = "0x00 - OHCI HCCA Base"]
    pub hc_hcca: HC_HCCA,
    #[doc = "0x04 - OHCI Period Current ED Base"]
    pub hc_period_current_ed: HC_PERIOD_CURRENT_ED,
    #[doc = "0x08 - OHCI Control Head ED Base"]
    pub hc_control_head_ed: HC_CONTROL_HEAD_ED,
    #[doc = "0x0c - OHCI Control Current ED Base"]
    pub hc_control_current_ed: HC_CONTROL_CURRENT_ED,
    #[doc = "0x10 - OHCI Bulk Head ED Base"]
    pub hc_bulk_head_ed: HC_BULK_HEAD_ED,
    #[doc = "0x14 - OHCI Bulk Current ED Base"]
    pub hc_bulk_current_ed: HC_BULK_CURRENT_ED,
    #[doc = "0x18 - OHCI Done Head Base"]
    pub hc_done_head: HC_DONE_HEAD,
}
#[doc = "hc_hcca (rw) register accessor: an alias for `Reg<HC_HCCA_SPEC>`"]
pub type HC_HCCA = crate::Reg<hc_hcca::HC_HCCA_SPEC>;
#[doc = "OHCI HCCA Base"]
pub mod hc_hcca;
#[doc = "hc_period_current_ed (rw) register accessor: an alias for `Reg<HC_PERIOD_CURRENT_ED_SPEC>`"]
pub type HC_PERIOD_CURRENT_ED = crate::Reg<hc_period_current_ed::HC_PERIOD_CURRENT_ED_SPEC>;
#[doc = "OHCI Period Current ED Base"]
pub mod hc_period_current_ed;
#[doc = "hc_control_head_ed (rw) register accessor: an alias for `Reg<HC_CONTROL_HEAD_ED_SPEC>`"]
pub type HC_CONTROL_HEAD_ED = crate::Reg<hc_control_head_ed::HC_CONTROL_HEAD_ED_SPEC>;
#[doc = "OHCI Control Head ED Base"]
pub mod hc_control_head_ed;
#[doc = "hc_control_current_ed (rw) register accessor: an alias for `Reg<HC_CONTROL_CURRENT_ED_SPEC>`"]
pub type HC_CONTROL_CURRENT_ED = crate::Reg<hc_control_current_ed::HC_CONTROL_CURRENT_ED_SPEC>;
#[doc = "OHCI Control Current ED Base"]
pub mod hc_control_current_ed;
#[doc = "hc_bulk_head_ed (rw) register accessor: an alias for `Reg<HC_BULK_HEAD_ED_SPEC>`"]
pub type HC_BULK_HEAD_ED = crate::Reg<hc_bulk_head_ed::HC_BULK_HEAD_ED_SPEC>;
#[doc = "OHCI Bulk Head ED Base"]
pub mod hc_bulk_head_ed;
#[doc = "hc_bulk_current_ed (rw) register accessor: an alias for `Reg<HC_BULK_CURRENT_ED_SPEC>`"]
pub type HC_BULK_CURRENT_ED = crate::Reg<hc_bulk_current_ed::HC_BULK_CURRENT_ED_SPEC>;
#[doc = "OHCI Bulk Current ED Base"]
pub mod hc_bulk_current_ed;
#[doc = "hc_done_head (rw) register accessor: an alias for `Reg<HC_DONE_HEAD_SPEC>`"]
pub type HC_DONE_HEAD = crate::Reg<hc_done_head::HC_DONE_HEAD_SPEC>;
#[doc = "OHCI Done Head Base"]
pub mod hc_done_head;
