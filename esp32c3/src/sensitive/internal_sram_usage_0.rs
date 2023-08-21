#[doc = "Register `INTERNAL_SRAM_USAGE_0` reader"]
pub type R = crate::R<INTERNAL_SRAM_USAGE_0_SPEC>;
#[doc = "Register `INTERNAL_SRAM_USAGE_0` writer"]
pub type W = crate::W<INTERNAL_SRAM_USAGE_0_SPEC>;
#[doc = "Field `INTERNAL_SRAM_USAGE_LOCK` reader - internal_sram_usage_lock"]
pub type INTERNAL_SRAM_USAGE_LOCK_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_USAGE_LOCK` writer - internal_sram_usage_lock"]
pub type INTERNAL_SRAM_USAGE_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - internal_sram_usage_lock"]
    #[inline(always)]
    pub fn internal_sram_usage_lock(&self) -> INTERNAL_SRAM_USAGE_LOCK_R {
        INTERNAL_SRAM_USAGE_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_0")
            .field(
                "internal_sram_usage_lock",
                &format_args!("{}", self.internal_sram_usage_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERNAL_SRAM_USAGE_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - internal_sram_usage_lock"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_usage_lock(
        &mut self,
    ) -> INTERNAL_SRAM_USAGE_LOCK_W<INTERNAL_SRAM_USAGE_0_SPEC, 0> {
        INTERNAL_SRAM_USAGE_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERNAL_SRAM_USAGE_0_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`internal_sram_usage_0::R`](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`internal_sram_usage_0::W`](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_0 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
