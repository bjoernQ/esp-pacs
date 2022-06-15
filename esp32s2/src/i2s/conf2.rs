#[doc = "Register `CONF2` reader"]
pub struct R(crate::R<CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF2` writer"]
pub struct W(crate::W<CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF2_SPEC>;
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
impl From<crate::W<CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAMERA_EN` reader - Set this bit to enable camera mode."]
pub type CAMERA_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAMERA_EN` writer - Set this bit to enable camera mode."]
pub type CAMERA_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 0>;
#[doc = "Field `LCD_TX_WRX2_EN` reader - LCD WR double for one datum."]
pub type LCD_TX_WRX2_EN_R = crate::BitReader<bool>;
#[doc = "Field `LCD_TX_WRX2_EN` writer - LCD WR double for one datum."]
pub type LCD_TX_WRX2_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 1>;
#[doc = "Field `LCD_TX_SDX2_EN` reader - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
pub type LCD_TX_SDX2_EN_R = crate::BitReader<bool>;
#[doc = "Field `LCD_TX_SDX2_EN` writer - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
pub type LCD_TX_SDX2_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 2>;
#[doc = "Field `DATA_ENABLE_TEST_EN` reader - for debug camera mode enable"]
pub type DATA_ENABLE_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `DATA_ENABLE_TEST_EN` writer - for debug camera mode enable"]
pub type DATA_ENABLE_TEST_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 3>;
#[doc = "Field `DATA_ENABLE` reader - for debug camera mode enable"]
pub type DATA_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DATA_ENABLE` writer - for debug camera mode enable"]
pub type DATA_ENABLE_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 4>;
#[doc = "Field `LCD_EN` reader - Set this bit to enable LCD mode."]
pub type LCD_EN_R = crate::BitReader<bool>;
#[doc = "Field `LCD_EN` writer - Set this bit to enable LCD mode."]
pub type LCD_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 5>;
#[doc = "Field `EXT_ADC_START_EN` reader - Set this bit to enable the function that ADC mode is triggered by external signal."]
pub type EXT_ADC_START_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXT_ADC_START_EN` writer - Set this bit to enable the function that ADC mode is triggered by external signal."]
pub type EXT_ADC_START_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 6>;
#[doc = "Field `INTER_VALID_EN` reader - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
pub type INTER_VALID_EN_R = crate::BitReader<bool>;
#[doc = "Field `INTER_VALID_EN` writer - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
pub type INTER_VALID_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 7>;
#[doc = "Field `CAM_SYNC_FIFO_RESET` reader - Set this bit to reset FIFO in camera mode."]
pub type CAM_SYNC_FIFO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CAM_SYNC_FIFO_RESET` writer - Set this bit to reset FIFO in camera mode."]
pub type CAM_SYNC_FIFO_RESET_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 8>;
#[doc = "Field `CAM_CLK_LOOPBACK` reader - Set this bit to loopback PCLK from I2S0I_WS_out."]
pub type CAM_CLK_LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `CAM_CLK_LOOPBACK` writer - Set this bit to loopback PCLK from I2S0I_WS_out."]
pub type CAM_CLK_LOOPBACK_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 9>;
#[doc = "Field `VSYNC_FILTER_EN` reader - Set this bit to enable I2S VSYNC filter function."]
pub type VSYNC_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC_FILTER_EN` writer - Set this bit to enable I2S VSYNC filter function."]
pub type VSYNC_FILTER_EN_W<'a> = crate::BitWriter<'a, u32, CONF2_SPEC, bool, 10>;
#[doc = "Field `VSYNC_FILTER_THRES` reader - Configure the I2S VSYNC filter threshold value."]
pub type VSYNC_FILTER_THRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSYNC_FILTER_THRES` writer - Configure the I2S VSYNC filter threshold value."]
pub type VSYNC_FILTER_THRES_W<'a> = crate::FieldWriter<'a, u32, CONF2_SPEC, u8, u8, 3, 11>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable camera mode."]
    #[inline(always)]
    pub fn camera_en(&self) -> CAMERA_EN_R {
        CAMERA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD WR double for one datum."]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&self) -> LCD_TX_WRX2_EN_R {
        LCD_TX_WRX2_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&self) -> LCD_TX_SDX2_EN_R {
        LCD_TX_SDX2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - for debug camera mode enable"]
    #[inline(always)]
    pub fn data_enable_test_en(&self) -> DATA_ENABLE_TEST_EN_R {
        DATA_ENABLE_TEST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - for debug camera mode enable"]
    #[inline(always)]
    pub fn data_enable(&self) -> DATA_ENABLE_R {
        DATA_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable LCD mode."]
    #[inline(always)]
    pub fn lcd_en(&self) -> LCD_EN_R {
        LCD_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable the function that ADC mode is triggered by external signal."]
    #[inline(always)]
    pub fn ext_adc_start_en(&self) -> EXT_ADC_START_EN_R {
        EXT_ADC_START_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
    #[inline(always)]
    pub fn inter_valid_en(&self) -> INTER_VALID_EN_R {
        INTER_VALID_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to reset FIFO in camera mode."]
    #[inline(always)]
    pub fn cam_sync_fifo_reset(&self) -> CAM_SYNC_FIFO_RESET_R {
        CAM_SYNC_FIFO_RESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to loopback PCLK from I2S0I_WS_out."]
    #[inline(always)]
    pub fn cam_clk_loopback(&self) -> CAM_CLK_LOOPBACK_R {
        CAM_CLK_LOOPBACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable I2S VSYNC filter function."]
    #[inline(always)]
    pub fn vsync_filter_en(&self) -> VSYNC_FILTER_EN_R {
        VSYNC_FILTER_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Configure the I2S VSYNC filter threshold value."]
    #[inline(always)]
    pub fn vsync_filter_thres(&self) -> VSYNC_FILTER_THRES_R {
        VSYNC_FILTER_THRES_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable camera mode."]
    #[inline(always)]
    pub fn camera_en(&mut self) -> CAMERA_EN_W {
        CAMERA_EN_W::new(self)
    }
    #[doc = "Bit 1 - LCD WR double for one datum."]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&mut self) -> LCD_TX_WRX2_EN_W {
        LCD_TX_WRX2_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&mut self) -> LCD_TX_SDX2_EN_W {
        LCD_TX_SDX2_EN_W::new(self)
    }
    #[doc = "Bit 3 - for debug camera mode enable"]
    #[inline(always)]
    pub fn data_enable_test_en(&mut self) -> DATA_ENABLE_TEST_EN_W {
        DATA_ENABLE_TEST_EN_W::new(self)
    }
    #[doc = "Bit 4 - for debug camera mode enable"]
    #[inline(always)]
    pub fn data_enable(&mut self) -> DATA_ENABLE_W {
        DATA_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to enable LCD mode."]
    #[inline(always)]
    pub fn lcd_en(&mut self) -> LCD_EN_W {
        LCD_EN_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to enable the function that ADC mode is triggered by external signal."]
    #[inline(always)]
    pub fn ext_adc_start_en(&mut self) -> EXT_ADC_START_EN_W {
        EXT_ADC_START_EN_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
    #[inline(always)]
    pub fn inter_valid_en(&mut self) -> INTER_VALID_EN_W {
        INTER_VALID_EN_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to reset FIFO in camera mode."]
    #[inline(always)]
    pub fn cam_sync_fifo_reset(&mut self) -> CAM_SYNC_FIFO_RESET_W {
        CAM_SYNC_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to loopback PCLK from I2S0I_WS_out."]
    #[inline(always)]
    pub fn cam_clk_loopback(&mut self) -> CAM_CLK_LOOPBACK_W {
        CAM_CLK_LOOPBACK_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to enable I2S VSYNC filter function."]
    #[inline(always)]
    pub fn vsync_filter_en(&mut self) -> VSYNC_FILTER_EN_W {
        VSYNC_FILTER_EN_W::new(self)
    }
    #[doc = "Bits 11:13 - Configure the I2S VSYNC filter threshold value."]
    #[inline(always)]
    pub fn vsync_filter_thres(&mut self) -> VSYNC_FILTER_THRES_W {
        VSYNC_FILTER_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf2](index.html) module"]
pub struct CONF2_SPEC;
impl crate::RegisterSpec for CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf2::R](R) reader structure"]
impl crate::Readable for CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf2::W](W) writer structure"]
impl crate::Writable for CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF2 to value 0"]
impl crate::Resettable for CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
