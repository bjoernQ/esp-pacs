#[doc = "Register `SET_CONTINUE` writer"]
pub type W = crate::W<SET_CONTINUE_SPEC>;
#[doc = "Field `SET_CONTINUE` writer - set this bit to continue DS operation."]
pub type SET_CONTINUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - set this bit to continue DS operation."]
    #[inline(always)]
    #[must_use]
    pub fn set_continue(&mut self) -> SET_CONTINUE_W<SET_CONTINUE_SPEC, 0> {
        SET_CONTINUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DS continue control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_CONTINUE_SPEC;
impl crate::RegisterSpec for SET_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_continue::W`](W) writer structure"]
impl crate::Writable for SET_CONTINUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_CONTINUE to value 0"]
impl crate::Resettable for SET_CONTINUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
