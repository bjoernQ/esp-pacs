#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - SPI0 control register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - SPI0 control1 register."]
    pub ctrl1: CTRL1,
    #[doc = "0x10 - SPI0 control2 register."]
    pub ctrl2: CTRL2,
    #[doc = "0x14 - SPI clock division control register."]
    pub clock: CLOCK,
    #[doc = "0x18 - SPI0 user register."]
    pub user: USER,
    #[doc = "0x1c - SPI0 user1 register."]
    pub user1: USER1,
    #[doc = "0x20 - SPI0 user2 register."]
    pub user2: USER2,
    _reserved7: [u8; 0x08],
    #[doc = "0x2c - SPI0 read control register."]
    pub rd_status: RD_STATUS,
    _reserved8: [u8; 0x04],
    #[doc = "0x34 - SPI0 misc register"]
    pub misc: MISC,
    _reserved9: [u8; 0x04],
    #[doc = "0x3c - SPI0 bit mode control register."]
    pub cache_fctrl: CACHE_FCTRL,
    _reserved10: [u8; 0x14],
    #[doc = "0x54 - SPI0 FSM status register"]
    pub fsm: FSM,
    _reserved11: [u8; 0x50],
    #[doc = "0xa8 - SPI0 timing calibration register"]
    pub timing_cali: TIMING_CALI,
    #[doc = "0xac - SPI0 input delay mode control register"]
    pub din_mode: DIN_MODE,
    #[doc = "0xb0 - SPI0 input delay number control register"]
    pub din_num: DIN_NUM,
    #[doc = "0xb4 - SPI0 output delay mode control register"]
    pub dout_mode: DOUT_MODE,
    _reserved15: [u8; 0x24],
    #[doc = "0xdc - SPI0 clk_gate register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0xe0 - SPI0 module clock select register"]
    pub core_clk_sel: CORE_CLK_SEL,
    _reserved17: [u8; 0x0318],
    #[doc = "0x3fc - Version control register"]
    pub date: DATE,
}
#[doc = "CTRL (rw) register accessor: SPI0 control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI0 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI0 control1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI0 control1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: SPI0 control2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI0 control2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI clock division control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI clock division control register."]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI0 user register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI0 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI0 user1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI0 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI0 user2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI0 user2 register."]
pub mod user2;
#[doc = "RD_STATUS (rw) register accessor: SPI0 read control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI0 read control register."]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: SPI0 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI0 misc register"]
pub mod misc;
#[doc = "CACHE_FCTRL (rw) register accessor: SPI0 bit mode control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI0 bit mode control register."]
pub mod cache_fctrl;
#[doc = "FSM (rw) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsm`] module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod fsm;
#[doc = "TIMING_CALI (r) register accessor: SPI0 timing calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timing_cali`] module"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI0 timing calibration register"]
pub mod timing_cali;
#[doc = "DIN_MODE (r) register accessor: SPI0 input delay mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`din_mode`] module"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "SPI0 input delay mode control register"]
pub mod din_mode;
#[doc = "DIN_NUM (r) register accessor: SPI0 input delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`din_num`] module"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "SPI0 input delay number control register"]
pub mod din_num;
#[doc = "DOUT_MODE (r) register accessor: SPI0 output delay mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dout_mode`] module"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "SPI0 output delay mode control register"]
pub mod dout_mode;
#[doc = "CLOCK_GATE (rw) register accessor: SPI0 clk_gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI0 clk_gate register"]
pub mod clock_gate;
#[doc = "CORE_CLK_SEL (rw) register accessor: SPI0 module clock select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_clk_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_clk_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_clk_sel`] module"]
pub type CORE_CLK_SEL = crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>;
#[doc = "SPI0 module clock select register"]
pub mod core_clk_sel;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
