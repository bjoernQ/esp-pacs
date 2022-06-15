#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_CLR` writer - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 0>;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 1>;
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - Clear SDIO idle interrupt state"]
pub type SDIO_IDLE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 2>;
#[doc = "Field `WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type WDT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 3>;
#[doc = "Field `TIME_VALID_INT_CLR` writer - Clear RTC time valid interrupt state"]
pub type TIME_VALID_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 4>;
#[doc = "Field `SAR_INT_CLR` writer - Clear ULP-coprocessor interrupt state"]
pub type SAR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 5>;
#[doc = "Field `TOUCH_INT_CLR` writer - Clear touch interrupt state"]
pub type TOUCH_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 6>;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 7>;
#[doc = "Field `MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 8>;
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W {
        SLP_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W {
        SLP_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Clear SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W {
        SDIO_IDLE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W {
        WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Clear RTC time valid interrupt state"]
    #[inline(always)]
    pub fn time_valid_int_clr(&mut self) -> TIME_VALID_INT_CLR_W {
        TIME_VALID_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Clear ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn sar_int_clr(&mut self) -> SAR_INT_CLR_W {
        SAR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Clear touch interrupt state"]
    #[inline(always)]
    pub fn touch_int_clr(&mut self) -> TOUCH_INT_CLR_W {
        TOUCH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W {
        BROWN_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W {
        MAIN_TIMER_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
