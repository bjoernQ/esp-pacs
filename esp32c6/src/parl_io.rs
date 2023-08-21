#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Parallel RX module configuration register0."]
    pub rx_cfg0: RX_CFG0,
    #[doc = "0x04 - Parallel RX module configuration register1."]
    pub rx_cfg1: RX_CFG1,
    #[doc = "0x08 - Parallel TX module configuration register0."]
    pub tx_cfg0: TX_CFG0,
    #[doc = "0x0c - Parallel TX module configuration register1."]
    pub tx_cfg1: TX_CFG1,
    #[doc = "0x10 - Parallel IO module status register0."]
    pub st: ST,
    #[doc = "0x14 - Parallel IO interrupt enable singal configuration register."]
    pub int_ena: INT_ENA,
    #[doc = "0x18 - Parallel IO interrupt raw singal status register."]
    pub int_raw: INT_RAW,
    #[doc = "0x1c - Parallel IO interrupt singal status register."]
    pub int_st: INT_ST,
    #[doc = "0x20 - Parallel IO interrupt clear singal configuration register."]
    pub int_clr: INT_CLR,
    _reserved9: [u8; 0xfc],
    #[doc = "0x120 - Parallel IO clk configuration register"]
    pub clk: CLK,
    _reserved10: [u8; 0x02d8],
    #[doc = "0x3fc - Version register."]
    pub version: VERSION,
}
#[doc = "RX_CFG0 (rw) register accessor: Parallel RX module configuration register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_cfg0`] module"]
pub type RX_CFG0 = crate::Reg<rx_cfg0::RX_CFG0_SPEC>;
#[doc = "Parallel RX module configuration register0."]
pub mod rx_cfg0;
#[doc = "RX_CFG1 (rw) register accessor: Parallel RX module configuration register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_cfg1`] module"]
pub type RX_CFG1 = crate::Reg<rx_cfg1::RX_CFG1_SPEC>;
#[doc = "Parallel RX module configuration register1."]
pub mod rx_cfg1;
#[doc = "TX_CFG0 (rw) register accessor: Parallel TX module configuration register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_cfg0`] module"]
pub type TX_CFG0 = crate::Reg<tx_cfg0::TX_CFG0_SPEC>;
#[doc = "Parallel TX module configuration register0."]
pub mod tx_cfg0;
#[doc = "TX_CFG1 (rw) register accessor: Parallel TX module configuration register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_cfg1`] module"]
pub type TX_CFG1 = crate::Reg<tx_cfg1::TX_CFG1_SPEC>;
#[doc = "Parallel TX module configuration register1."]
pub mod tx_cfg1;
#[doc = "ST (r) register accessor: Parallel IO module status register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st`] module"]
pub type ST = crate::Reg<st::ST_SPEC>;
#[doc = "Parallel IO module status register0."]
pub mod st;
#[doc = "INT_ENA (rw) register accessor: Parallel IO interrupt enable singal configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Parallel IO interrupt enable singal configuration register."]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Parallel IO interrupt raw singal status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Parallel IO interrupt raw singal status register."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Parallel IO interrupt singal status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Parallel IO interrupt singal status register."]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Parallel IO interrupt clear singal configuration register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Parallel IO interrupt clear singal configuration register."]
pub mod int_clr;
#[doc = "CLK (rw) register accessor: Parallel IO clk configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Parallel IO clk configuration register"]
pub mod clk;
#[doc = "VERSION (rw) register accessor: Version register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version register."]
pub mod version;
