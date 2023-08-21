#[doc = "Register `CLKGATE_FORCE_ON` reader"]
pub type R = crate::R<CLKGATE_FORCE_ON_SPEC>;
#[doc = "Register `CLKGATE_FORCE_ON` writer"]
pub type W = crate::W<CLKGATE_FORCE_ON_SPEC>;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - ******* Description ***********"]
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - ******* Description ***********"]
pub type ROM_CLKGATE_FORCE_ON_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` reader - ******* Description ***********"]
pub type SRAM_CLKGATE_FORCE_ON_R = crate::FieldReader<u16>;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` writer - ******* Description ***********"]
pub type SRAM_CLKGATE_FORCE_ON_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:2 - ******* Description ***********"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKGATE_FORCE_ON")
            .field(
                "rom_clkgate_force_on",
                &format_args!("{}", self.rom_clkgate_force_on().bits()),
            )
            .field(
                "sram_clkgate_force_on",
                &format_args!("{}", self.sram_clkgate_force_on().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKGATE_FORCE_ON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<CLKGATE_FORCE_ON_SPEC, 0> {
        ROM_CLKGATE_FORCE_ON_W::new(self)
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W<CLKGATE_FORCE_ON_SPEC, 3> {
        SRAM_CLKGATE_FORCE_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_force_on::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_force_on::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKGATE_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLKGATE_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_force_on::R`](R) reader structure"]
impl crate::Readable for CLKGATE_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkgate_force_on::W`](W) writer structure"]
impl crate::Writable for CLKGATE_FORCE_ON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKGATE_FORCE_ON to value 0x3fff"]
impl crate::Resettable for CLKGATE_FORCE_ON_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
