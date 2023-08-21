#[doc = "Register `SIGMADELTA1` reader"]
pub type R = crate::R<SIGMADELTA1_SPEC>;
#[doc = "Register `SIGMADELTA1` writer"]
pub type W = crate::W<SIGMADELTA1_SPEC>;
#[doc = "Field `SD1_IN` reader - "]
pub type SD1_IN_R = crate::FieldReader;
#[doc = "Field `SD1_IN` writer - "]
pub type SD1_IN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SD1_PRESCALE` reader - "]
pub type SD1_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD1_PRESCALE` writer - "]
pub type SD1_PRESCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd1_in(&self) -> SD1_IN_R {
        SD1_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd1_prescale(&self) -> SD1_PRESCALE_R {
        SD1_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA1")
            .field("sd1_in", &format_args!("{}", self.sd1_in().bits()))
            .field(
                "sd1_prescale",
                &format_args!("{}", self.sd1_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd1_in(&mut self) -> SD1_IN_W<SIGMADELTA1_SPEC, 0> {
        SD1_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd1_prescale(&mut self) -> SD1_PRESCALE_W<SIGMADELTA1_SPEC, 8> {
        SD1_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA1_SPEC;
impl crate::RegisterSpec for SIGMADELTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta1::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta1::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA1 to value 0xff00"]
impl crate::Resettable for SIGMADELTA1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
