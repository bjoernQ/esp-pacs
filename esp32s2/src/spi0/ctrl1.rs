#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type CLK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type CLK_MODE_W<'a> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 2, 0>;
#[doc = "Field `CLK_MODE_13` reader - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type CLK_MODE_13_R = crate::BitReader<bool>;
#[doc = "Field `CLK_MODE_13` writer - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type CLK_MODE_13_W<'a> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, 2>;
#[doc = "Field `RSCK_DATA_OUT` reader - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type RSCK_DATA_OUT_R = crate::BitReader<bool>;
#[doc = "Field `RSCK_DATA_OUT` writer - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type RSCK_DATA_OUT_W<'a> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, 3>;
#[doc = "Field `W16_17_WR_ENA` reader - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
pub type W16_17_WR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `W16_17_WR_ENA` writer - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
pub type W16_17_WR_ENA_W<'a> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, 4>;
#[doc = "Field `CS_HOLD_DELAY` reader - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
pub type CS_HOLD_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS_HOLD_DELAY` writer - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
pub type CS_HOLD_DELAY_W<'a> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 6, 14>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
    #[inline(always)]
    pub fn w16_17_wr_ena(&self) -> W16_17_WR_ENA_R {
        W16_17_WR_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 14:19 - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W {
        CLK_MODE_W::new(self)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W {
        CLK_MODE_13_W::new(self)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W {
        RSCK_DATA_OUT_W::new(self)
    }
    #[doc = "Bit 4 - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
    #[inline(always)]
    pub fn w16_17_wr_ena(&mut self) -> W16_17_WR_ENA_W {
        W16_17_WR_ENA_W::new(self)
    }
    #[doc = "Bits 14:19 - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W {
        CS_HOLD_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0x4010"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4010
    }
}
