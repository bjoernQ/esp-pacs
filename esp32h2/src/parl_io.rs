#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Parallel RX Sampling mode configuration register."]
    pub rx_mode_cfg: RX_MODE_CFG,
    #[doc = "0x04 - Parallel RX data configuration register."]
    pub rx_data_cfg: RX_DATA_CFG,
    #[doc = "0x08 - Parallel RX general configuration register."]
    pub rx_genrl_cfg: RX_GENRL_CFG,
    #[doc = "0x0c - Parallel RX Start configuration register."]
    pub rx_start_cfg: RX_START_CFG,
    #[doc = "0x10 - Parallel TX data configuration register."]
    pub tx_data_cfg: TX_DATA_CFG,
    #[doc = "0x14 - Parallel TX Start configuration register."]
    pub tx_start_cfg: TX_START_CFG,
    #[doc = "0x18 - Parallel TX general configuration register."]
    pub tx_genrl_cfg: TX_GENRL_CFG,
    #[doc = "0x1c - Parallel IO FIFO configuration register."]
    pub fifo_cfg: FIFO_CFG,
    #[doc = "0x20 - Parallel IO FIFO configuration register."]
    pub reg_update: REG_UPDATE,
    #[doc = "0x24 - Parallel IO module status register0."]
    pub st: ST,
    #[doc = "0x28 - Parallel IO interrupt enable singal configuration register."]
    pub int_ena: INT_ENA,
    #[doc = "0x2c - Parallel IO interrupt raw singal status register."]
    pub int_raw: INT_RAW,
    #[doc = "0x30 - Parallel IO interrupt singal status register."]
    pub int_st: INT_ST,
    #[doc = "0x34 - Parallel IO interrupt clear singal configuration register."]
    pub int_clr: INT_CLR,
    #[doc = "0x38 - Parallel IO RX status register0"]
    pub rx_st0: RX_ST0,
    #[doc = "0x3c - Parallel IO RX status register1"]
    pub rx_st1: RX_ST1,
    #[doc = "0x40 - Parallel IO TX status register0"]
    pub tx_st0: TX_ST0,
    #[doc = "0x44 - Parallel IO RX clk configuration register"]
    pub rx_clk_cfg: RX_CLK_CFG,
    #[doc = "0x48 - Parallel IO TX clk configuration register"]
    pub tx_clk_cfg: TX_CLK_CFG,
    _reserved19: [u8; 0xd4],
    #[doc = "0x120 - Parallel IO clk configuration register"]
    pub clk: CLK,
    _reserved20: [u8; 0x02d8],
    #[doc = "0x3fc - Version register."]
    pub version: VERSION,
}
#[doc = "RX_MODE_CFG (rw) register accessor: Parallel RX Sampling mode configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_mode_cfg`] module"]
pub type RX_MODE_CFG = crate::Reg<rx_mode_cfg::RX_MODE_CFG_SPEC>;
#[doc = "Parallel RX Sampling mode configuration register."]
pub mod rx_mode_cfg;
#[doc = "RX_DATA_CFG (rw) register accessor: Parallel RX data configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_data_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_data_cfg`] module"]
pub type RX_DATA_CFG = crate::Reg<rx_data_cfg::RX_DATA_CFG_SPEC>;
#[doc = "Parallel RX data configuration register."]
pub mod rx_data_cfg;
#[doc = "RX_GENRL_CFG (rw) register accessor: Parallel RX general configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_genrl_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_genrl_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_genrl_cfg`] module"]
pub type RX_GENRL_CFG = crate::Reg<rx_genrl_cfg::RX_GENRL_CFG_SPEC>;
#[doc = "Parallel RX general configuration register."]
pub mod rx_genrl_cfg;
#[doc = "RX_START_CFG (rw) register accessor: Parallel RX Start configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_start_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_start_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_start_cfg`] module"]
pub type RX_START_CFG = crate::Reg<rx_start_cfg::RX_START_CFG_SPEC>;
#[doc = "Parallel RX Start configuration register."]
pub mod rx_start_cfg;
#[doc = "TX_DATA_CFG (rw) register accessor: Parallel TX data configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_data_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_data_cfg`] module"]
pub type TX_DATA_CFG = crate::Reg<tx_data_cfg::TX_DATA_CFG_SPEC>;
#[doc = "Parallel TX data configuration register."]
pub mod tx_data_cfg;
#[doc = "TX_START_CFG (rw) register accessor: Parallel TX Start configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_start_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_start_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_start_cfg`] module"]
pub type TX_START_CFG = crate::Reg<tx_start_cfg::TX_START_CFG_SPEC>;
#[doc = "Parallel TX Start configuration register."]
pub mod tx_start_cfg;
#[doc = "TX_GENRL_CFG (rw) register accessor: Parallel TX general configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_genrl_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_genrl_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_genrl_cfg`] module"]
pub type TX_GENRL_CFG = crate::Reg<tx_genrl_cfg::TX_GENRL_CFG_SPEC>;
#[doc = "Parallel TX general configuration register."]
pub mod tx_genrl_cfg;
#[doc = "FIFO_CFG (rw) register accessor: Parallel IO FIFO configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo_cfg`] module"]
pub type FIFO_CFG = crate::Reg<fifo_cfg::FIFO_CFG_SPEC>;
#[doc = "Parallel IO FIFO configuration register."]
pub mod fifo_cfg;
#[doc = "REG_UPDATE (w) register accessor: Parallel IO FIFO configuration register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reg_update`] module"]
pub type REG_UPDATE = crate::Reg<reg_update::REG_UPDATE_SPEC>;
#[doc = "Parallel IO FIFO configuration register."]
pub mod reg_update;
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
#[doc = "RX_ST0 (r) register accessor: Parallel IO RX status register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_st0`] module"]
pub type RX_ST0 = crate::Reg<rx_st0::RX_ST0_SPEC>;
#[doc = "Parallel IO RX status register0"]
pub mod rx_st0;
#[doc = "RX_ST1 (r) register accessor: Parallel IO RX status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_st1`] module"]
pub type RX_ST1 = crate::Reg<rx_st1::RX_ST1_SPEC>;
#[doc = "Parallel IO RX status register1"]
pub mod rx_st1;
#[doc = "TX_ST0 (r) register accessor: Parallel IO TX status register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_st0`] module"]
pub type TX_ST0 = crate::Reg<tx_st0::TX_ST0_SPEC>;
#[doc = "Parallel IO TX status register0"]
pub mod tx_st0;
#[doc = "RX_CLK_CFG (rw) register accessor: Parallel IO RX clk configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_clk_cfg`] module"]
pub type RX_CLK_CFG = crate::Reg<rx_clk_cfg::RX_CLK_CFG_SPEC>;
#[doc = "Parallel IO RX clk configuration register"]
pub mod rx_clk_cfg;
#[doc = "TX_CLK_CFG (rw) register accessor: Parallel IO TX clk configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_clk_cfg`] module"]
pub type TX_CLK_CFG = crate::Reg<tx_clk_cfg::TX_CLK_CFG_SPEC>;
#[doc = "Parallel IO TX clk configuration register"]
pub mod tx_clk_cfg;
#[doc = "CLK (rw) register accessor: Parallel IO clk configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Parallel IO clk configuration register"]
pub mod clk;
#[doc = "VERSION (rw) register accessor: Version register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version register."]
pub mod version;
