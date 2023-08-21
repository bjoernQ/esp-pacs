#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configure the low level width of SCL"]
    pub scl_low: SCL_LOW,
    #[doc = "0x04 - Transmission setting"]
    pub ctrl: CTRL,
    #[doc = "0x08 - RTC I2C status"]
    pub status: STATUS,
    #[doc = "0x0c - Configure RTC I2C timeout"]
    pub to: TO,
    #[doc = "0x10 - Configure slave address"]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x14 - Configure the high level width of SCL"]
    pub scl_high: SCL_HIGH,
    #[doc = "0x18 - Configure the SDA hold time after a negative SCL edge"]
    pub sda_duty: SDA_DUTY,
    #[doc = "0x1c - Configure the delay between the SDA and SCL negative edge for a start condition"]
    pub scl_start_period: SCL_START_PERIOD,
    #[doc = "0x20 - Configure the delay between SDA and SCL positive edge for a stop condition"]
    pub scl_stop_period: SCL_STOP_PERIOD,
    #[doc = "0x24 - Clear RTC I2C interrupt"]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - RTC I2C raw interrupt"]
    pub int_raw: INT_RAW,
    #[doc = "0x2c - RTC I2C interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x30 - Enable RTC I2C interrupt"]
    pub int_ena: INT_ENA,
    #[doc = "0x34 - RTC I2C read data"]
    pub data: DATA,
    #[doc = "0x38..0x78 - RTC I2C Command %s"]
    pub cmd: [CMD; 16],
    _reserved15: [u8; 0x84],
    #[doc = "0xfc - Version control register"]
    pub date: DATE,
}
#[doc = "SCL_LOW (rw) register accessor: Configure the low level width of SCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_low`] module"]
pub type SCL_LOW = crate::Reg<scl_low::SCL_LOW_SPEC>;
#[doc = "Configure the low level width of SCL"]
pub mod scl_low;
#[doc = "CTRL (rw) register accessor: Transmission setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Transmission setting"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: RTC I2C status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "RTC I2C status"]
pub mod status;
#[doc = "TO (rw) register accessor: Configure RTC I2C timeout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`to`] module"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "Configure RTC I2C timeout"]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: Configure slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave_addr`] module"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "Configure slave address"]
pub mod slave_addr;
#[doc = "SCL_HIGH (rw) register accessor: Configure the high level width of SCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_high`] module"]
pub type SCL_HIGH = crate::Reg<scl_high::SCL_HIGH_SPEC>;
#[doc = "Configure the high level width of SCL"]
pub mod scl_high;
#[doc = "SDA_DUTY (rw) register accessor: Configure the SDA hold time after a negative SCL edge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sda_duty`] module"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = "Configure the SDA hold time after a negative SCL edge"]
pub mod sda_duty;
#[doc = "SCL_START_PERIOD (rw) register accessor: Configure the delay between the SDA and SCL negative edge for a start condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_start_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_start_period`] module"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = "Configure the delay between the SDA and SCL negative edge for a start condition"]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD (rw) register accessor: Configure the delay between SDA and SCL positive edge for a stop condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_stop_period`] module"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = "Configure the delay between SDA and SCL positive edge for a stop condition"]
pub mod scl_stop_period;
#[doc = "INT_CLR (w) register accessor: Clear RTC I2C interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Clear RTC I2C interrupt"]
pub mod int_clr;
#[doc = "INT_RAW (r) register accessor: RTC I2C raw interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RTC I2C raw interrupt"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: RTC I2C interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RTC I2C interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Enable RTC I2C interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Enable RTC I2C interrupt"]
pub mod int_ena;
#[doc = "DATA (rw) register accessor: RTC I2C read data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "RTC I2C read data"]
pub mod data;
#[doc = "CMD (rw) register accessor: RTC I2C Command %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "RTC I2C Command %s"]
pub mod cmd;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
