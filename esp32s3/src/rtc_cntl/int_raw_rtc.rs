#[doc = "Register `INT_RAW_RTC` reader"]
pub struct R(crate::R<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW_RTC` writer"]
pub struct W(crate::W<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_RTC_SPEC>;
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
impl From<crate::W<INT_RAW_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_RAW` reader - sleep wakeup interrupt raw"]
pub struct SLP_WAKEUP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SLP_WAKEUP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT_INT_RAW` reader - sleep reject interrupt raw"]
pub struct SLP_REJECT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SLP_REJECT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_IDLE_INT_RAW` reader - SDIO idle interrupt raw"]
pub struct SDIO_IDLE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SDIO_IDLE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_IDLE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_IDLE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_WDT_INT_RAW` reader - RTC WDT interrupt raw"]
pub struct RTC_WDT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_WDT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_WDT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_WDT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_SCAN_DONE_INT_RAW` reader - enable touch scan done interrupt raw"]
pub struct RTC_TOUCH_SCAN_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_SCAN_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_SCAN_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_SCAN_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ULP_CP_INT_RAW` reader - ULP-coprocessor interrupt raw"]
pub struct RTC_ULP_CP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_ULP_CP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ULP_CP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ULP_CP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_DONE_INT_RAW` reader - touch interrupt raw"]
pub struct RTC_TOUCH_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_ACTIVE_INT_RAW` reader - touch active interrupt raw"]
pub struct RTC_TOUCH_ACTIVE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_ACTIVE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_ACTIVE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_ACTIVE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_INACTIVE_INT_RAW` reader - touch inactive interrupt raw"]
pub struct RTC_TOUCH_INACTIVE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_INACTIVE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_INACTIVE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_INACTIVE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_BROWN_OUT_INT_RAW` reader - brown out interrupt raw"]
pub struct RTC_BROWN_OUT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_BROWN_OUT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_BROWN_OUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_BROWN_OUT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MAIN_TIMER_INT_RAW` reader - RTC main timer interrupt raw"]
pub struct RTC_MAIN_TIMER_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_MAIN_TIMER_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MAIN_TIMER_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MAIN_TIMER_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SARADC1_INT_RAW` reader - saradc1 interrupt raw"]
pub struct RTC_SARADC1_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_SARADC1_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SARADC1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SARADC1_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TSENS_INT_RAW` reader - tsens interrupt raw"]
pub struct RTC_TSENS_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_TSENS_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TSENS_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TSENS_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_COCPU_INT_RAW` reader - riscV cocpu interrupt raw"]
pub struct RTC_COCPU_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_COCPU_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_COCPU_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_COCPU_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SARADC2_INT_RAW` reader - saradc2 interrupt raw"]
pub struct RTC_SARADC2_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_SARADC2_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SARADC2_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SARADC2_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SWD_INT_RAW` reader - super watch dog interrupt raw"]
pub struct RTC_SWD_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_SWD_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SWD_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SWD_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_XTAL32K_DEAD_INT_RAW` reader - xtal32k dead detection interrupt raw"]
pub struct RTC_XTAL32K_DEAD_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_XTAL32K_DEAD_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_XTAL32K_DEAD_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_XTAL32K_DEAD_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_COCPU_TRAP_INT_RAW` reader - cocpu trap interrupt raw"]
pub struct RTC_COCPU_TRAP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_COCPU_TRAP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_COCPU_TRAP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_COCPU_TRAP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_TIMEOUT_INT_RAW` reader - touch timeout interrupt raw"]
pub struct RTC_TOUCH_TIMEOUT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_TIMEOUT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_TIMEOUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_TIMEOUT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GLITCH_DET_INT_RAW` reader - glitch_det_interrupt_raw"]
pub struct RTC_GLITCH_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_GLITCH_DET_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GLITCH_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GLITCH_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW` reader - touch approach mode loop interrupt raw"]
pub struct RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW` writer - touch approach mode loop interrupt raw"]
pub struct RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&self) -> SLP_WAKEUP_INT_RAW_R {
        SLP_WAKEUP_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&self) -> SLP_REJECT_INT_RAW_R {
        SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt raw"]
    #[inline(always)]
    pub fn sdio_idle_int_raw(&self) -> SDIO_IDLE_INT_RAW_R {
        SDIO_IDLE_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn rtc_wdt_int_raw(&self) -> RTC_WDT_INT_RAW_R {
        RTC_WDT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_scan_done_int_raw(&self) -> RTC_TOUCH_SCAN_DONE_INT_RAW_R {
        RTC_TOUCH_SCAN_DONE_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt raw"]
    #[inline(always)]
    pub fn rtc_ulp_cp_int_raw(&self) -> RTC_ULP_CP_INT_RAW_R {
        RTC_ULP_CP_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - touch interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_done_int_raw(&self) -> RTC_TOUCH_DONE_INT_RAW_R {
        RTC_TOUCH_DONE_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - touch active interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_active_int_raw(&self) -> RTC_TOUCH_ACTIVE_INT_RAW_R {
        RTC_TOUCH_ACTIVE_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - touch inactive interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_inactive_int_raw(&self) -> RTC_TOUCH_INACTIVE_INT_RAW_R {
        RTC_TOUCH_INACTIVE_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn rtc_brown_out_int_raw(&self) -> RTC_BROWN_OUT_INT_RAW_R {
        RTC_BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn rtc_main_timer_int_raw(&self) -> RTC_MAIN_TIMER_INT_RAW_R {
        RTC_MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - saradc1 interrupt raw"]
    #[inline(always)]
    pub fn rtc_saradc1_int_raw(&self) -> RTC_SARADC1_INT_RAW_R {
        RTC_SARADC1_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - tsens interrupt raw"]
    #[inline(always)]
    pub fn rtc_tsens_int_raw(&self) -> RTC_TSENS_INT_RAW_R {
        RTC_TSENS_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - riscV cocpu interrupt raw"]
    #[inline(always)]
    pub fn rtc_cocpu_int_raw(&self) -> RTC_COCPU_INT_RAW_R {
        RTC_COCPU_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - saradc2 interrupt raw"]
    #[inline(always)]
    pub fn rtc_saradc2_int_raw(&self) -> RTC_SARADC2_INT_RAW_R {
        RTC_SARADC2_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn rtc_swd_int_raw(&self) -> RTC_SWD_INT_RAW_R {
        RTC_SWD_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt raw"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_raw(&self) -> RTC_XTAL32K_DEAD_INT_RAW_R {
        RTC_XTAL32K_DEAD_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - cocpu trap interrupt raw"]
    #[inline(always)]
    pub fn rtc_cocpu_trap_int_raw(&self) -> RTC_COCPU_TRAP_INT_RAW_R {
        RTC_COCPU_TRAP_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - touch timeout interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_timeout_int_raw(&self) -> RTC_TOUCH_TIMEOUT_INT_RAW_R {
        RTC_TOUCH_TIMEOUT_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt_raw"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_raw(&self) -> RTC_GLITCH_DET_INT_RAW_R {
        RTC_GLITCH_DET_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_raw(&self) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - touch approach mode loop interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_raw(
        &mut self,
    ) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_W {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc interrupt register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_rtc]
(index.html) module"]
pub struct INT_RAW_RTC_SPEC;
impl crate::RegisterSpec for INT_RAW_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_rtc::R]
(R) reader structure"]
impl crate::Readable for INT_RAW_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw_rtc::W]
(W) writer structure"]
impl crate::Writable for INT_RAW_RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RAW_RTC to value 0"]
impl crate::Resettable for INT_RAW_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}