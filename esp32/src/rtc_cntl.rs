#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub options0: OPTIONS0,
    #[doc = "0x04 - "]
    pub slp_timer0: SLP_TIMER0,
    #[doc = "0x08 - "]
    pub slp_timer1: SLP_TIMER1,
    #[doc = "0x0c - "]
    pub time_update: TIME_UPDATE,
    #[doc = "0x10 - "]
    pub time0: TIME0,
    #[doc = "0x14 - "]
    pub time1: TIME1,
    #[doc = "0x18 - "]
    pub state0: STATE0,
    #[doc = "0x1c - "]
    pub timer1: TIMER1,
    #[doc = "0x20 - "]
    pub timer2: TIMER2,
    #[doc = "0x24 - "]
    pub timer3: TIMER3,
    #[doc = "0x28 - "]
    pub timer4: TIMER4,
    #[doc = "0x2c - "]
    pub timer5: TIMER5,
    #[doc = "0x30 - "]
    pub ana_conf: ANA_CONF,
    #[doc = "0x34 - "]
    pub reset_state: RESET_STATE,
    #[doc = "0x38 - "]
    pub wakeup_state: WAKEUP_STATE,
    #[doc = "0x3c - "]
    pub int_ena: INT_ENA,
    #[doc = "0x40 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x44 - "]
    pub int_st: INT_ST,
    #[doc = "0x48 - "]
    pub int_clr: INT_CLR,
    #[doc = "0x4c - "]
    pub store0: STORE0,
    #[doc = "0x50 - "]
    pub store1: STORE1,
    #[doc = "0x54 - "]
    pub store2: STORE2,
    #[doc = "0x58 - "]
    pub store3: STORE3,
    #[doc = "0x5c - "]
    pub ext_xtl_conf: EXT_XTL_CONF,
    #[doc = "0x60 - "]
    pub ext_wakeup_conf: EXT_WAKEUP_CONF,
    #[doc = "0x64 - "]
    pub slp_reject_conf: SLP_REJECT_CONF,
    #[doc = "0x68 - "]
    pub cpu_period_conf: CPU_PERIOD_CONF,
    #[doc = "0x6c - "]
    pub sdio_act_conf: SDIO_ACT_CONF,
    #[doc = "0x70 - "]
    pub clk_conf: CLK_CONF,
    #[doc = "0x74 - "]
    pub sdio_conf: SDIO_CONF,
    #[doc = "0x78 - "]
    pub bias_conf: BIAS_CONF,
    #[doc = "0x7c - "]
    pub reg: REG,
    #[doc = "0x80 - "]
    pub pwc: PWC,
    #[doc = "0x84 - "]
    pub dig_pwc: DIG_PWC,
    #[doc = "0x88 - "]
    pub dig_iso: DIG_ISO,
    #[doc = "0x8c - "]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x90 - "]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x94 - "]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x98 - "]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0x9c - "]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0xa0 - "]
    pub wdtfeed: WDTFEED,
    #[doc = "0xa4 - "]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0xa8 - "]
    pub test_mux: TEST_MUX,
    #[doc = "0xac - "]
    pub sw_cpu_stall: SW_CPU_STALL,
    #[doc = "0xb0 - "]
    pub store4: STORE4,
    #[doc = "0xb4 - "]
    pub store5: STORE5,
    #[doc = "0xb8 - "]
    pub store6: STORE6,
    #[doc = "0xbc - "]
    pub store7: STORE7,
    #[doc = "0xc0 - "]
    pub low_power_st: LOW_POWER_ST,
    #[doc = "0xc4 - "]
    pub diag1: DIAG1,
    #[doc = "0xc8 - "]
    pub hold_force: HOLD_FORCE,
    #[doc = "0xcc - "]
    pub ext_wakeup1: EXT_WAKEUP1,
    #[doc = "0xd0 - "]
    pub ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    #[doc = "0xd4 - "]
    pub brown_out: BROWN_OUT,
    _reserved54: [u8; 0x64],
    #[doc = "0x13c - "]
    pub date: DATE,
}
#[doc = "OPTIONS0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`options0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`options0`] module"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = ""]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_timer0`] module"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = ""]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_timer1`] module"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = ""]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_update`] module"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = ""]
pub mod time_update;
#[doc = "TIME0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time0`] module"]
pub type TIME0 = crate::Reg<time0::TIME0_SPEC>;
#[doc = ""]
pub mod time0;
#[doc = "TIME1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time1`] module"]
pub type TIME1 = crate::Reg<time1::TIME1_SPEC>;
#[doc = ""]
pub mod time1;
#[doc = "STATE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = ""]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer1`] module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = ""]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer2`] module"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = ""]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer3`] module"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = ""]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer4`] module"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = ""]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer5`] module"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = ""]
pub mod timer5;
#[doc = "ANA_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ana_conf`] module"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = ""]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reset_state`] module"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = ""]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wakeup_state`] module"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = ""]
pub mod wakeup_state;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "STORE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = ""]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store1`] module"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = ""]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store2`] module"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = ""]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store3`] module"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = ""]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_xtl_conf`] module"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = ""]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup_conf`] module"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = ""]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_reject_conf`] module"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = ""]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_period_conf`] module"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = ""]
pub mod cpu_period_conf;
#[doc = "SDIO_ACT_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_act_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_act_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_act_conf`] module"]
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
#[doc = ""]
pub mod sdio_act_conf;
#[doc = "CLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = ""]
pub mod clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_conf`] module"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = ""]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bias_conf`] module"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = ""]
pub mod bias_conf;
#[doc = "REG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reg`] module"]
pub type REG = crate::Reg<reg::REG_SPEC>;
#[doc = ""]
pub mod reg;
#[doc = "PWC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwc`] module"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = ""]
pub mod pwc;
#[doc = "DIG_PWC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_pwc`] module"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = ""]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_iso`] module"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = ""]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = ""]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = ""]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = ""]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = ""]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = ""]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = ""]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = ""]
pub mod wdtwprotect;
#[doc = "TEST_MUX (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`test_mux`] module"]
pub type TEST_MUX = crate::Reg<test_mux::TEST_MUX_SPEC>;
#[doc = ""]
pub mod test_mux;
#[doc = "SW_CPU_STALL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sw_cpu_stall`] module"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = ""]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store4`] module"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = ""]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store5`] module"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = ""]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store6`] module"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = ""]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store7`] module"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = ""]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`low_power_st`] module"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = ""]
pub mod low_power_st;
#[doc = "DIAG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diag1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diag1`] module"]
pub type DIAG1 = crate::Reg<diag1::DIAG1_SPEC>;
#[doc = ""]
pub mod diag1;
#[doc = "HOLD_FORCE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hold_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hold_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hold_force`] module"]
pub type HOLD_FORCE = crate::Reg<hold_force::HOLD_FORCE_SPEC>;
#[doc = ""]
pub mod hold_force;
#[doc = "EXT_WAKEUP1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup1`] module"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = ""]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup1_status`] module"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = ""]
pub mod ext_wakeup1_status;
#[doc = "BROWN_OUT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`brown_out`] module"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = ""]
pub mod brown_out;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
