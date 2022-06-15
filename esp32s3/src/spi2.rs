#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Command control register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x04 - Address value register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x08 - SPI control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - SPI clock control register"]
    pub clock: crate::Reg<clock::CLOCK_SPEC>,
    #[doc = "0x10 - SPI USER control register"]
    pub user: crate::Reg<user::USER_SPEC>,
    #[doc = "0x14 - SPI USER control register 1"]
    pub user1: crate::Reg<user1::USER1_SPEC>,
    #[doc = "0x18 - SPI USER control register 2"]
    pub user2: crate::Reg<user2::USER2_SPEC>,
    #[doc = "0x1c - SPI data bit length control register"]
    pub ms_dlen: crate::Reg<ms_dlen::MS_DLEN_SPEC>,
    #[doc = "0x20 - SPI misc register"]
    pub misc: crate::Reg<misc::MISC_SPEC>,
    #[doc = "0x24 - SPI input delay mode configuration"]
    pub din_mode: crate::Reg<din_mode::DIN_MODE_SPEC>,
    #[doc = "0x28 - SPI input delay number configuration"]
    pub din_num: crate::Reg<din_num::DIN_NUM_SPEC>,
    #[doc = "0x2c - SPI output delay mode configuration"]
    pub dout_mode: crate::Reg<dout_mode::DOUT_MODE_SPEC>,
    #[doc = "0x30 - SPI DMA control register"]
    pub dma_conf: crate::Reg<dma_conf::DMA_CONF_SPEC>,
    #[doc = "0x34 - SPI interrupt enable register"]
    pub dma_int_ena: crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>,
    #[doc = "0x38 - SPI interrupt clear register"]
    pub dma_int_clr: crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>,
    #[doc = "0x3c - SPI interrupt raw register"]
    pub dma_int_raw: crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>,
    #[doc = "0x40 - SPI interrupt status register"]
    pub dma_int_st: crate::Reg<dma_int_st::DMA_INT_ST_SPEC>,
    #[doc = "0x44 - SPI interrupt software set register"]
    pub dma_int_set: crate::Reg<dma_int_set::DMA_INT_SET_SPEC>,
    _reserved18: [u8; 0x50],
    #[doc = "0x98 - SPI CPU-controlled buffer0"]
    pub w0: crate::Reg<w0::W0_SPEC>,
    #[doc = "0x9c - SPI CPU-controlled buffer1"]
    pub w1: crate::Reg<w1::W1_SPEC>,
    #[doc = "0xa0 - SPI CPU-controlled buffer2"]
    pub w2: crate::Reg<w2::W2_SPEC>,
    #[doc = "0xa4 - SPI CPU-controlled buffer3"]
    pub w3: crate::Reg<w3::W3_SPEC>,
    #[doc = "0xa8 - SPI CPU-controlled buffer4"]
    pub w4: crate::Reg<w4::W4_SPEC>,
    #[doc = "0xac - SPI CPU-controlled buffer5"]
    pub w5: crate::Reg<w5::W5_SPEC>,
    #[doc = "0xb0 - SPI CPU-controlled buffer6"]
    pub w6: crate::Reg<w6::W6_SPEC>,
    #[doc = "0xb4 - SPI CPU-controlled buffer7"]
    pub w7: crate::Reg<w7::W7_SPEC>,
    #[doc = "0xb8 - SPI CPU-controlled buffer8"]
    pub w8: crate::Reg<w8::W8_SPEC>,
    #[doc = "0xbc - SPI CPU-controlled buffer9"]
    pub w9: crate::Reg<w9::W9_SPEC>,
    #[doc = "0xc0 - SPI CPU-controlled buffer10"]
    pub w10: crate::Reg<w10::W10_SPEC>,
    #[doc = "0xc4 - SPI CPU-controlled buffer11"]
    pub w11: crate::Reg<w11::W11_SPEC>,
    #[doc = "0xc8 - SPI CPU-controlled buffer12"]
    pub w12: crate::Reg<w12::W12_SPEC>,
    #[doc = "0xcc - SPI CPU-controlled buffer13"]
    pub w13: crate::Reg<w13::W13_SPEC>,
    #[doc = "0xd0 - SPI CPU-controlled buffer14"]
    pub w14: crate::Reg<w14::W14_SPEC>,
    #[doc = "0xd4 - SPI CPU-controlled buffer15"]
    pub w15: crate::Reg<w15::W15_SPEC>,
    _reserved34: [u8; 0x08],
    #[doc = "0xe0 - SPI slave control register"]
    pub slave: crate::Reg<slave::SLAVE_SPEC>,
    #[doc = "0xe4 - SPI slave control register 1"]
    pub slave1: crate::Reg<slave1::SLAVE1_SPEC>,
    #[doc = "0xe8 - SPI module clock and register clock control"]
    pub clk_gate: crate::Reg<clk_gate::CLK_GATE_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0xf0 - Version control"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command control register"]
pub mod cmd;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address value register"]
pub mod addr;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI control register"]
pub mod ctrl;
#[doc = "CLOCK register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI clock control register"]
pub mod clock;
#[doc = "USER register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI USER control register"]
pub mod user;
#[doc = "USER1 register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI USER control register 1"]
pub mod user1;
#[doc = "USER2 register accessor: an alias for `Reg<USER2_SPEC>`"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI USER control register 2"]
pub mod user2;
#[doc = "MS_DLEN register accessor: an alias for `Reg<MS_DLEN_SPEC>`"]
pub type MS_DLEN = crate::Reg<ms_dlen::MS_DLEN_SPEC>;
#[doc = "SPI data bit length control register"]
pub mod ms_dlen;
#[doc = "MISC register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI misc register"]
pub mod misc;
#[doc = "DIN_MODE register accessor: an alias for `Reg<DIN_MODE_SPEC>`"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "SPI input delay mode configuration"]
pub mod din_mode;
#[doc = "DIN_NUM register accessor: an alias for `Reg<DIN_NUM_SPEC>`"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "SPI input delay number configuration"]
pub mod din_num;
#[doc = "DOUT_MODE register accessor: an alias for `Reg<DOUT_MODE_SPEC>`"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "SPI output delay mode configuration"]
pub mod dout_mode;
#[doc = "DMA_CONF register accessor: an alias for `Reg<DMA_CONF_SPEC>`"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "SPI DMA control register"]
pub mod dma_conf;
#[doc = "DMA_INT_ENA register accessor: an alias for `Reg<DMA_INT_ENA_SPEC>`"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = "SPI interrupt enable register"]
pub mod dma_int_ena;
#[doc = "DMA_INT_CLR register accessor: an alias for `Reg<DMA_INT_CLR_SPEC>`"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = "SPI interrupt clear register"]
pub mod dma_int_clr;
#[doc = "DMA_INT_RAW register accessor: an alias for `Reg<DMA_INT_RAW_SPEC>`"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = "SPI interrupt raw register"]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST register accessor: an alias for `Reg<DMA_INT_ST_SPEC>`"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = "SPI interrupt status register"]
pub mod dma_int_st;
#[doc = "DMA_INT_SET register accessor: an alias for `Reg<DMA_INT_SET_SPEC>`"]
pub type DMA_INT_SET = crate::Reg<dma_int_set::DMA_INT_SET_SPEC>;
#[doc = "SPI interrupt software set register"]
pub mod dma_int_set;
#[doc = "W0 register accessor: an alias for `Reg<W0_SPEC>`"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = "SPI CPU-controlled buffer0"]
pub mod w0;
#[doc = "W1 register accessor: an alias for `Reg<W1_SPEC>`"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = "SPI CPU-controlled buffer1"]
pub mod w1;
#[doc = "W2 register accessor: an alias for `Reg<W2_SPEC>`"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = "SPI CPU-controlled buffer2"]
pub mod w2;
#[doc = "W3 register accessor: an alias for `Reg<W3_SPEC>`"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = "SPI CPU-controlled buffer3"]
pub mod w3;
#[doc = "W4 register accessor: an alias for `Reg<W4_SPEC>`"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = "SPI CPU-controlled buffer4"]
pub mod w4;
#[doc = "W5 register accessor: an alias for `Reg<W5_SPEC>`"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = "SPI CPU-controlled buffer5"]
pub mod w5;
#[doc = "W6 register accessor: an alias for `Reg<W6_SPEC>`"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = "SPI CPU-controlled buffer6"]
pub mod w6;
#[doc = "W7 register accessor: an alias for `Reg<W7_SPEC>`"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = "SPI CPU-controlled buffer7"]
pub mod w7;
#[doc = "W8 register accessor: an alias for `Reg<W8_SPEC>`"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = "SPI CPU-controlled buffer8"]
pub mod w8;
#[doc = "W9 register accessor: an alias for `Reg<W9_SPEC>`"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = "SPI CPU-controlled buffer9"]
pub mod w9;
#[doc = "W10 register accessor: an alias for `Reg<W10_SPEC>`"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = "SPI CPU-controlled buffer10"]
pub mod w10;
#[doc = "W11 register accessor: an alias for `Reg<W11_SPEC>`"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = "SPI CPU-controlled buffer11"]
pub mod w11;
#[doc = "W12 register accessor: an alias for `Reg<W12_SPEC>`"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = "SPI CPU-controlled buffer12"]
pub mod w12;
#[doc = "W13 register accessor: an alias for `Reg<W13_SPEC>`"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = "SPI CPU-controlled buffer13"]
pub mod w13;
#[doc = "W14 register accessor: an alias for `Reg<W14_SPEC>`"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = "SPI CPU-controlled buffer14"]
pub mod w14;
#[doc = "W15 register accessor: an alias for `Reg<W15_SPEC>`"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = "SPI CPU-controlled buffer15"]
pub mod w15;
#[doc = "SLAVE register accessor: an alias for `Reg<SLAVE_SPEC>`"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = "SPI slave control register"]
pub mod slave;
#[doc = "SLAVE1 register accessor: an alias for `Reg<SLAVE1_SPEC>`"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = "SPI slave control register 1"]
pub mod slave1;
#[doc = "CLK_GATE register accessor: an alias for `Reg<CLK_GATE_SPEC>`"]
pub type CLK_GATE = crate::Reg<clk_gate::CLK_GATE_SPEC>;
#[doc = "SPI module clock and register clock control"]
pub mod clk_gate;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control"]
pub mod date;
