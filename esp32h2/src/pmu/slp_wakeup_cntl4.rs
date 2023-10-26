#[doc = "Register `SLP_WAKEUP_CNTL4` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL4_SPEC>;
#[doc = "Field `SLP_REJECT_CAUSE_CLR` writer - need_des"]
pub type SLP_REJECT_CAUSE_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_cause_clr(&mut self) -> SLP_REJECT_CAUSE_CLR_W<SLP_WAKEUP_CNTL4_SPEC, 31> {
        SLP_REJECT_CAUSE_CLR_W::new(self)
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
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL4_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl4::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL4 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
