#[doc = "Register `OUT_W1TS` writer"]
pub type W = crate::W<OUT_W1TS_SPEC>;
#[doc = "Field `OUT_W1TS` writer - GPIO output set register for GPIO0-31"]
pub type OUT_W1TS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO output set register for GPIO0-31"]
    #[inline(always)]
    #[must_use]
    pub fn out_w1ts(&mut self) -> OUT_W1TS_W<OUT_W1TS_SPEC, 0> {
        OUT_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO output set register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_W1TS_SPEC;
impl crate::RegisterSpec for OUT_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_w1ts::W`](W) writer structure"]
impl crate::Writable for OUT_W1TS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_W1TS to value 0"]
impl crate::Resettable for OUT_W1TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
