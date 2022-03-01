#[doc = "Register `MISC` reader"]
pub struct R(crate::R<MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC` writer"]
pub struct W(crate::W<MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0_DIS` reader - SPI CS0 pin enable, 1: disable CS0, 0: SPI_CS0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub struct CS0_DIS_R(crate::FieldReader<bool, bool>);
impl CS0_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS0_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS0_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS0_DIS` writer - SPI CS0 pin enable, 1: disable CS0, 0: SPI_CS0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub struct CS0_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CS1_DIS` reader - SPI CS1 pin enable, 1: disable CS1, 0: SPI_CS1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub struct CS1_DIS_R(crate::FieldReader<bool, bool>);
impl CS1_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS1_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1_DIS` writer - SPI CS1 pin enable, 1: disable CS1, 0: SPI_CS1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub struct CS1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CS2_DIS` reader - SPI CS2 pin enable, 1: disable CS2, 0: SPI_CS2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub struct CS2_DIS_R(crate::FieldReader<bool, bool>);
impl CS2_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS2_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS2_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS2_DIS` writer - SPI CS2 pin enable, 1: disable CS2, 0: SPI_CS2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub struct CS2_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS2_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CS3_DIS` reader - SPI CS3 pin enable, 1: disable CS3, 0: SPI_CS3 signal is from/to CS3 pin. Can be configured in CONF state."]
pub struct CS3_DIS_R(crate::FieldReader<bool, bool>);
impl CS3_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS3_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS3_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS3_DIS` writer - SPI CS3 pin enable, 1: disable CS3, 0: SPI_CS3 signal is from/to CS3 pin. Can be configured in CONF state."]
pub struct CS3_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS3_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CS4_DIS` reader - SPI CS4 pin enable, 1: disable CS4, 0: SPI_CS4 signal is from/to CS4 pin. Can be configured in CONF state."]
pub struct CS4_DIS_R(crate::FieldReader<bool, bool>);
impl CS4_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS4_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS4_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS4_DIS` writer - SPI CS4 pin enable, 1: disable CS4, 0: SPI_CS4 signal is from/to CS4 pin. Can be configured in CONF state."]
pub struct CS4_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS4_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CS5_DIS` reader - SPI CS5 pin enable, 1: disable CS5, 0: SPI_CS5 signal is from/to CS5 pin. Can be configured in CONF state."]
pub struct CS5_DIS_R(crate::FieldReader<bool, bool>);
impl CS5_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS5_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS5_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS5_DIS` writer - SPI CS5 pin enable, 1: disable CS5, 0: SPI_CS5 signal is from/to CS5 pin. Can be configured in CONF state."]
pub struct CS5_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS5_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CK_DIS` reader - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub struct CK_DIS_R(crate::FieldReader<bool, bool>);
impl CK_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_DIS` writer - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub struct CK_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
pub struct MASTER_CS_POL_R(crate::FieldReader<u8, u8>);
impl MASTER_CS_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASTER_CS_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_CS_POL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
pub struct MASTER_CS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_CS_POL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `CLK_DATA_DTR_EN` reader - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub struct CLK_DATA_DTR_EN_R(crate::FieldReader<bool, bool>);
impl CLK_DATA_DTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_DATA_DTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DATA_DTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DATA_DTR_EN` writer - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub struct CLK_DATA_DTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DATA_DTR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `DATA_DTR_EN` reader - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub struct DATA_DTR_EN_R(crate::FieldReader<bool, bool>);
impl DATA_DTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_DTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_DTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_DTR_EN` writer - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub struct DATA_DTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_DTR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ADDR_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub struct ADDR_DTR_EN_R(crate::FieldReader<bool, bool>);
impl ADDR_DTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDR_DTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_DTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_DTR_EN` writer - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub struct ADDR_DTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_DTR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CMD_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub struct CMD_DTR_EN_R(crate::FieldReader<bool, bool>);
impl CMD_DTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_DTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_DTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_DTR_EN` writer - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub struct CMD_DTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_DTR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CD_DATA_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_DATA_SET_R(crate::FieldReader<bool, bool>);
impl CD_DATA_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CD_DATA_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_DATA_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD_DATA_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_DATA_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_DATA_SET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CD_DUMMY_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_DUMMY_SET_R(crate::FieldReader<bool, bool>);
impl CD_DUMMY_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CD_DUMMY_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_DUMMY_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD_DUMMY_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_DUMMY_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_DUMMY_SET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CD_ADDR_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_ADDR_SET_R(crate::FieldReader<bool, bool>);
impl CD_ADDR_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CD_ADDR_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_ADDR_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD_ADDR_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_ADDR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_ADDR_SET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `SLAVE_CS_POL` reader - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub struct SLAVE_CS_POL_R(crate::FieldReader<bool, bool>);
impl SLAVE_CS_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_CS_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_CS_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_CS_POL` writer - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub struct SLAVE_CS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_CS_POL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `DQS_IDLE_EDGE` reader - The default value of spi_dqs. Can be configured in CONF state."]
pub struct DQS_IDLE_EDGE_R(crate::FieldReader<bool, bool>);
impl DQS_IDLE_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DQS_IDLE_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQS_IDLE_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQS_IDLE_EDGE` writer - The default value of spi_dqs. Can be configured in CONF state."]
pub struct DQS_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DQS_IDLE_EDGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CD_CMD_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_CMD_SET_R(crate::FieldReader<bool, bool>);
impl CD_CMD_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CD_CMD_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_CMD_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD_CMD_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub struct CD_CMD_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_CMD_SET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CD_IDLE_EDGE` reader - The default value of spi_cd. Can be configured in CONF state."]
pub struct CD_IDLE_EDGE_R(crate::FieldReader<bool, bool>);
impl CD_IDLE_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CD_IDLE_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_IDLE_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD_IDLE_EDGE` writer - The default value of spi_cd. Can be configured in CONF state."]
pub struct CD_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_IDLE_EDGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub struct CK_IDLE_EDGE_R(crate::FieldReader<bool, bool>);
impl CK_IDLE_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK_IDLE_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_IDLE_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub struct CK_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_IDLE_EDGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub struct CS_KEEP_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CS_KEEP_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS_KEEP_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_KEEP_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub struct CS_KEEP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_KEEP_ACTIVE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `QUAD_DIN_PIN_SWAP` reader - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub struct QUAD_DIN_PIN_SWAP_R(crate::FieldReader<bool, bool>);
impl QUAD_DIN_PIN_SWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUAD_DIN_PIN_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUAD_DIN_PIN_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUAD_DIN_PIN_SWAP` writer - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub struct QUAD_DIN_PIN_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAD_DIN_PIN_SWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: SPI_CS0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: SPI_CS1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: SPI_CS2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS2_DIS_R {
        CS2_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SPI CS3 pin enable, 1: disable CS3, 0: SPI_CS3 signal is from/to CS3 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs3_dis(&self) -> CS3_DIS_R {
        CS3_DIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI CS4 pin enable, 1: disable CS4, 0: SPI_CS4 signal is from/to CS4 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs4_dis(&self) -> CS4_DIS_R {
        CS4_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI CS5 pin enable, 1: disable CS5, 0: SPI_CS5 signal is from/to CS5 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs5_dis(&self) -> CS5_DIS_R {
        CS5_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
    #[inline(always)]
    pub fn clk_data_dtr_en(&self) -> CLK_DATA_DTR_EN_R {
        CLK_DATA_DTR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn data_dtr_en(&self) -> DATA_DTR_EN_R {
        DATA_DTR_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn addr_dtr_en(&self) -> ADDR_DTR_EN_R {
        ADDR_DTR_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cmd_dtr_en(&self) -> CMD_DTR_EN_R {
        CMD_DTR_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_data_set(&self) -> CD_DATA_SET_R {
        CD_DATA_SET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_dummy_set(&self) -> CD_DUMMY_SET_R {
        CD_DUMMY_SET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_addr_set(&self) -> CD_ADDR_SET_R {
        CD_ADDR_SET_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn slave_cs_pol(&self) -> SLAVE_CS_POL_R {
        SLAVE_CS_POL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dqs_idle_edge(&self) -> DQS_IDLE_EDGE_R {
        DQS_IDLE_EDGE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_cmd_set(&self) -> CD_CMD_SET_R {
        CD_CMD_SET_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The default value of spi_cd. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_idle_edge(&self) -> CD_IDLE_EDGE_R {
        CD_IDLE_EDGE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn quad_din_pin_swap(&self) -> QUAD_DIN_PIN_SWAP_R {
        QUAD_DIN_PIN_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: SPI_CS0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W {
        CS0_DIS_W { w: self }
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: SPI_CS1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W {
        CS1_DIS_W { w: self }
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: SPI_CS2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs2_dis(&mut self) -> CS2_DIS_W {
        CS2_DIS_W { w: self }
    }
    #[doc = "Bit 3 - SPI CS3 pin enable, 1: disable CS3, 0: SPI_CS3 signal is from/to CS3 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs3_dis(&mut self) -> CS3_DIS_W {
        CS3_DIS_W { w: self }
    }
    #[doc = "Bit 4 - SPI CS4 pin enable, 1: disable CS4, 0: SPI_CS4 signal is from/to CS4 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs4_dis(&mut self) -> CS4_DIS_W {
        CS4_DIS_W { w: self }
    }
    #[doc = "Bit 5 - SPI CS5 pin enable, 1: disable CS5, 0: SPI_CS5 signal is from/to CS5 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs5_dis(&mut self) -> CS5_DIS_W {
        CS5_DIS_W { w: self }
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_dis(&mut self) -> CK_DIS_W {
        CK_DIS_W { w: self }
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
    #[inline(always)]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W {
        MASTER_CS_POL_W { w: self }
    }
    #[doc = "Bit 16 - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
    #[inline(always)]
    pub fn clk_data_dtr_en(&mut self) -> CLK_DATA_DTR_EN_W {
        CLK_DATA_DTR_EN_W { w: self }
    }
    #[doc = "Bit 17 - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn data_dtr_en(&mut self) -> DATA_DTR_EN_W {
        DATA_DTR_EN_W { w: self }
    }
    #[doc = "Bit 18 - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn addr_dtr_en(&mut self) -> ADDR_DTR_EN_W {
        ADDR_DTR_EN_W { w: self }
    }
    #[doc = "Bit 19 - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cmd_dtr_en(&mut self) -> CMD_DTR_EN_W {
        CMD_DTR_EN_W { w: self }
    }
    #[doc = "Bit 20 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_data_set(&mut self) -> CD_DATA_SET_W {
        CD_DATA_SET_W { w: self }
    }
    #[doc = "Bit 21 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_dummy_set(&mut self) -> CD_DUMMY_SET_W {
        CD_DUMMY_SET_W { w: self }
    }
    #[doc = "Bit 22 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_addr_set(&mut self) -> CD_ADDR_SET_W {
        CD_ADDR_SET_W { w: self }
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn slave_cs_pol(&mut self) -> SLAVE_CS_POL_W {
        SLAVE_CS_POL_W { w: self }
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dqs_idle_edge(&mut self) -> DQS_IDLE_EDGE_W {
        DQS_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 25 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\]
 is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_cmd_set(&mut self) -> CD_CMD_SET_W {
        CD_CMD_SET_W { w: self }
    }
    #[doc = "Bit 26 - The default value of spi_cd. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_idle_edge(&mut self) -> CD_IDLE_EDGE_W {
        CD_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W {
        CK_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W {
        CS_KEEP_ACTIVE_W { w: self }
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn quad_din_pin_swap(&mut self) -> QUAD_DIN_PIN_SWAP_W {
        QUAD_DIN_PIN_SWAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI misc register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc]
(index.html) module"]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc::R]
(R) reader structure"]
impl crate::Readable for MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc::W]
(W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC to value 0x3e"]
impl crate::Resettable for MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3e
    }
}