#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB OTG Wrapper Configure Register"]
    pub otg_conf: crate::Reg<otg_conf::OTG_CONF_SPEC>,
    #[doc = "0x04 - USB Internal PHY Testing Register"]
    pub test_conf: crate::Reg<test_conf::TEST_CONF_SPEC>,
    _reserved2: [u8; 0x03f4],
    #[doc = "0x3fc - Version Control Register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "OTG_CONF register accessor: an alias for `Reg<OTG_CONF_SPEC>`"]
pub type OTG_CONF = crate::Reg<otg_conf::OTG_CONF_SPEC>;
#[doc = "USB OTG Wrapper Configure Register"]
pub mod otg_conf;
#[doc = "TEST_CONF register accessor: an alias for `Reg<TEST_CONF_SPEC>`"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = "USB Internal PHY Testing Register"]
pub mod test_conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version Control Register"]
pub mod date;