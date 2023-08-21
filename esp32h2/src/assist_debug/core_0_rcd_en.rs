#[doc = "Register `CORE_0_RCD_EN` reader"]
pub type R = crate::R<CORE_0_RCD_EN_SPEC>;
#[doc = "Register `CORE_0_RCD_EN` writer"]
pub type W = crate::W<CORE_0_RCD_EN_SPEC>;
#[doc = "Field `CORE_0_RCD_RECORDEN` reader - Set 1 to enable record PC"]
pub type CORE_0_RCD_RECORDEN_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_RECORDEN` writer - Set 1 to enable record PC"]
pub type CORE_0_RCD_RECORDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` reader - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
pub type CORE_0_RCD_PDEBUGEN_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` writer - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
pub type CORE_0_RCD_PDEBUGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable record PC"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&self) -> CORE_0_RCD_RECORDEN_R {
        CORE_0_RCD_RECORDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&self) -> CORE_0_RCD_PDEBUGEN_R {
        CORE_0_RCD_PDEBUGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_EN")
            .field(
                "core_0_rcd_recorden",
                &format_args!("{}", self.core_0_rcd_recorden().bit()),
            )
            .field(
                "core_0_rcd_pdebugen",
                &format_args!("{}", self.core_0_rcd_pdebugen().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_RCD_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable record PC"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_rcd_recorden(&mut self) -> CORE_0_RCD_RECORDEN_W<CORE_0_RCD_EN_SPEC, 0> {
        CORE_0_RCD_RECORDEN_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_rcd_pdebugen(&mut self) -> CORE_0_RCD_PDEBUGEN_W<CORE_0_RCD_EN_SPEC, 1> {
        CORE_0_RCD_PDEBUGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "record enable configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_EN_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_en::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_rcd_en::W`](W) writer structure"]
impl crate::Writable for CORE_0_RCD_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_RCD_EN to value 0"]
impl crate::Resettable for CORE_0_RCD_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
