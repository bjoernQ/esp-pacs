#[doc = "Register `INT_ENA_RTC_W1TS` writer"]
pub type W = crate::W<INT_ENA_RTC_W1TS_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TS` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TS` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO_IDLE_INT_ENA_W1TS` writer - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_INT_ENA_W1TS` writer - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOUCH_SCAN_DONE_INT_ENA_W1TS` writer - enable touch scan done interrupt"]
pub type TOUCH_SCAN_DONE_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ULP_CP_INT_ENA_W1TS` writer - enable ULP-coprocessor interrupt"]
pub type ULP_CP_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOUCH_DONE_INT_ENA_W1TS` writer - enable touch done interrupt"]
pub type TOUCH_DONE_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOUCH_ACTIVE_INT_ENA_W1TS` writer - enable touch active interrupt"]
pub type TOUCH_ACTIVE_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOUCH_INACTIVE_INT_ENA_W1TS` writer - enable touch inactive interrupt"]
pub type TOUCH_INACTIVE_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BROWN_OUT_INT_ENA_W1TS` writer - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TS` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARADC1_INT_ENA_W1TS` writer - enable saradc1 interrupt"]
pub type SARADC1_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSENS_INT_ENA_W1TS` writer - enable tsens interrupt"]
pub type TSENS_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_INT_ENA_W1TS` writer - enable riscV cocpu interrupt"]
pub type COCPU_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARADC2_INT_ENA_W1TS` writer - enable saradc2 interrupt"]
pub type SARADC2_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWD_INT_ENA_W1TS` writer - enable super watch dog interrupt"]
pub type SWD_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_DEAD_INT_ENA_W1TS` writer - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_TRAP_INT_ENA_W1TS` writer - enable cocpu trap interrupt"]
pub type COCPU_TRAP_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOUCH_TIMEOUT_INT_ENA_W1TS` writer - enable touch timeout interrupt"]
pub type TOUCH_TIMEOUT_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GLITCH_DET_INT_ENA_W1TS` writer - enbale gitch det interrupt"]
pub type GLITCH_DET_INT_ENA_W1TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS` writer - enbale touch approach_loop done interrupt"]
pub type TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena_w1ts(
        &mut self,
    ) -> SLP_WAKEUP_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 0> {
        SLP_WAKEUP_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena_w1ts(
        &mut self,
    ) -> SLP_REJECT_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 1> {
        SLP_REJECT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_ena_w1ts(&mut self) -> SDIO_IDLE_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 2> {
        SDIO_IDLE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena_w1ts(&mut self) -> WDT_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 3> {
        WDT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_done_int_ena_w1ts(
        &mut self,
    ) -> TOUCH_SCAN_DONE_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 4> {
        TOUCH_SCAN_DONE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_int_ena_w1ts(&mut self) -> ULP_CP_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 5> {
        ULP_CP_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 6 - enable touch done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_done_int_ena_w1ts(
        &mut self,
    ) -> TOUCH_DONE_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 6> {
        TOUCH_DONE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 7 - enable touch active interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_active_int_ena_w1ts(
        &mut self,
    ) -> TOUCH_ACTIVE_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 7> {
        TOUCH_ACTIVE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 8 - enable touch inactive interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive_int_ena_w1ts(
        &mut self,
    ) -> TOUCH_INACTIVE_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 8> {
        TOUCH_INACTIVE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena_w1ts(&mut self) -> BROWN_OUT_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 9> {
        BROWN_OUT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena_w1ts(
        &mut self,
    ) -> MAIN_TIMER_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 10> {
        MAIN_TIMER_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 11 - enable saradc1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn saradc1_int_ena_w1ts(&mut self) -> SARADC1_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 11> {
        SARADC1_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 12 - enable tsens interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_int_ena_w1ts(&mut self) -> TSENS_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 12> {
        TSENS_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 13 - enable riscV cocpu interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_int_ena_w1ts(&mut self) -> COCPU_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 13> {
        COCPU_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 14 - enable saradc2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn saradc2_int_ena_w1ts(&mut self) -> SARADC2_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 14> {
        SARADC2_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_ena_w1ts(&mut self) -> SWD_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 15> {
        SWD_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_ena_w1ts(
        &mut self,
    ) -> XTAL32K_DEAD_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 16> {
        XTAL32K_DEAD_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 17 - enable cocpu trap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_trap_int_ena_w1ts(
        &mut self,
    ) -> COCPU_TRAP_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 17> {
        COCPU_TRAP_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 18 - enable touch timeout interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_int_ena_w1ts(
        &mut self,
    ) -> TOUCH_TIMEOUT_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 18> {
        TOUCH_TIMEOUT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_ena_w1ts(
        &mut self,
    ) -> GLITCH_DET_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 19> {
        GLITCH_DET_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 20 - enbale touch approach_loop done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_loop_done_int_ena_w1ts(
        &mut self,
    ) -> TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC, 20> {
        TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "oneset rtc interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc_w1ts::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TS to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
