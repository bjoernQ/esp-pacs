#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TRIGGER_SPEC>;
#[doc = "Field `TRIGGER` writer - Set to enable manual encryption."]
pub type TRIGGER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable manual encryption."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<TRIGGER_SPEC, 0> {
        TRIGGER_W::new(self)
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
#[doc = "Activates AES algorithm\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
