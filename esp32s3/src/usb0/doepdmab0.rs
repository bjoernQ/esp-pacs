#[doc = "Register `DOEPDMAB0` reader"]
pub type R = crate::R<DOEPDMAB0_SPEC>;
#[doc = "Register `DOEPDMAB0` writer"]
pub type W = crate::W<DOEPDMAB0_SPEC>;
#[doc = "Field `DMABUFFERADDR0` reader - "]
pub type DMABUFFERADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `DMABUFFERADDR0` writer - "]
pub type DMABUFFERADDR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr0(&self) -> DMABUFFERADDR0_R {
        DMABUFFERADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB0")
            .field(
                "dmabufferaddr0",
                &format_args!("{}", self.dmabufferaddr0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr0(&mut self) -> DMABUFFERADDR0_W<DOEPDMAB0_SPEC, 0> {
        DMABUFFERADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB0_SPEC;
impl crate::RegisterSpec for DOEPDMAB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab0::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdmab0::W`](W) writer structure"]
impl crate::Writable for DOEPDMAB0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMAB0 to value 0"]
impl crate::Resettable for DOEPDMAB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
