#[doc = "Register `INTERRUPT_REG_DATE` reader"]
pub type R = crate::R<INTERRUPT_REG_DATE_SPEC>;
#[doc = "Register `INTERRUPT_REG_DATE` writer"]
pub type W = crate::W<INTERRUPT_REG_DATE_SPEC>;
#[doc = "Field `INTERRUPT_REG_DATE` reader - Need add description"]
pub type INTERRUPT_REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `INTERRUPT_REG_DATE` writer - Need add description"]
pub type INTERRUPT_REG_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - Need add description"]
    #[inline(always)]
    pub fn interrupt_reg_date(&self) -> INTERRUPT_REG_DATE_R {
        INTERRUPT_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_REG_DATE")
            .field(
                "interrupt_reg_date",
                &format_args!("{}", self.interrupt_reg_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_REG_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_reg_date(&mut self) -> INTERRUPT_REG_DATE_W<INTERRUPT_REG_DATE_SPEC, 0> {
        INTERRUPT_REG_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_REG_DATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_reg_date::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_reg_date::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_REG_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPT_REG_DATE to value 0x0210_8190"]
impl crate::Resettable for INTERRUPT_REG_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210_8190;
}
