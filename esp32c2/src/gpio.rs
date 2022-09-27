#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO bit select register"]
    pub bt_select: BT_SELECT,
    #[doc = "0x04 - GPIO output register"]
    pub out: OUT,
    #[doc = "0x08 - GPIO output set register"]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x0c - GPIO output clear register"]
    pub out_w1tc: OUT_W1TC,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - GPIO sdio select register"]
    pub sdio_select: SDIO_SELECT,
    #[doc = "0x20 - GPIO output enable register"]
    pub enable: ENABLE,
    #[doc = "0x24 - GPIO output enable set register"]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x28 - GPIO output enable clear register"]
    pub enable_w1tc: ENABLE_W1TC,
    _reserved8: [u8; 0x0c],
    #[doc = "0x38 - pad strapping register"]
    pub strap: STRAP,
    #[doc = "0x3c - GPIO input register"]
    pub in_: IN,
    _reserved10: [u8; 0x04],
    #[doc = "0x44 - GPIO interrupt status register"]
    pub status: STATUS,
    #[doc = "0x48 - GPIO interrupt status set register"]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x4c - GPIO interrupt status clear register"]
    pub status_w1tc: STATUS_W1TC,
    _reserved13: [u8; 0x0c],
    #[doc = "0x5c - GPIO PRO_CPU interrupt status register"]
    pub pcpu_int: PCPU_INT,
    #[doc = "0x60 - GPIO PRO_CPU(not shielded) interrupt status register"]
    pub pcpu_nmi_int: PCPU_NMI_INT,
    #[doc = "0x64 - GPIO CPUSDIO interrupt status register"]
    pub cpusdio_int: CPUSDIO_INT,
    _reserved16: [u8; 0x0c],
    #[doc = "0x74..0xd8 - GPIO pin configuration register"]
    pub pin: [PIN; 25],
    _reserved17: [u8; 0x74],
    #[doc = "0x14c - GPIO interrupt source register"]
    pub status_next: STATUS_NEXT,
    _reserved18: [u8; 0x04],
    #[doc = "0x154..0x354 - GPIO input function configuration register"]
    pub func_in_sel_cfg: [FUNC_IN_SEL_CFG; 128],
    _reserved19: [u8; 0x0200],
    #[doc = "0x554..0x5b8 - GPIO output function select register"]
    pub func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 25],
    _reserved20: [u8; 0x74],
    #[doc = "0x62c - GPIO clock gate register"]
    pub clock_gate_reg: CLOCK_GATE_REG,
    _reserved21: [u8; 0xcc],
    #[doc = "0x6fc - GPIO version register"]
    pub reg_date_reg: REG_DATE_REG,
}
#[doc = "BT_SELECT (rw) register accessor: an alias for `Reg<BT_SELECT_SPEC>`"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = "GPIO bit select register"]
pub mod bt_select;
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register"]
pub mod out_w1tc;
#[doc = "SDIO_SELECT (rw) register accessor: an alias for `Reg<SDIO_SELECT_SPEC>`"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = "GPIO sdio select register"]
pub mod sdio_select;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register"]
pub mod enable_w1tc;
#[doc = "STRAP (r) register accessor: an alias for `Reg<STRAP_SPEC>`"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "pad strapping register"]
pub mod strap;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register"]
pub mod in_;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register"]
pub mod status_w1tc;
#[doc = "PCPU_INT (r) register accessor: an alias for `Reg<PCPU_INT_SPEC>`"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register"]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT (r) register accessor: an alias for `Reg<PCPU_NMI_INT_SPEC>`"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register"]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT (r) register accessor: an alias for `Reg<CPUSDIO_INT_SPEC>`"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register"]
pub mod cpusdio_int;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin;
#[doc = "STATUS_NEXT (r) register accessor: an alias for `Reg<STATUS_NEXT_SPEC>`"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register"]
pub mod status_next;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC_IN_SEL_CFG_SPEC>`"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC_OUT_SEL_CFG_SPEC>`"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func_out_sel_cfg;
#[doc = "CLOCK_GATE_REG (rw) register accessor: an alias for `Reg<CLOCK_GATE_REG_SPEC>`"]
pub type CLOCK_GATE_REG = crate::Reg<clock_gate_reg::CLOCK_GATE_REG_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate_reg;
#[doc = "REG_DATE_REG (rw) register accessor: an alias for `Reg<REG_DATE_REG_SPEC>`"]
pub type REG_DATE_REG = crate::Reg<reg_date_reg::REG_DATE_REG_SPEC>;
#[doc = "GPIO version register"]
pub mod reg_date_reg;