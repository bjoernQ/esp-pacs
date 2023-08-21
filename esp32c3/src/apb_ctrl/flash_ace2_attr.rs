#[doc = "Register `FLASH_ACE2_ATTR` reader"]
pub type R = crate::R<FLASH_ACE2_ATTR_SPEC>;
#[doc = "Register `FLASH_ACE2_ATTR` writer"]
pub type W = crate::W<FLASH_ACE2_ATTR_SPEC>;
#[doc = "Field `FLASH_ACE2_ATTR` reader - reg_flash_ace2_attr"]
pub type FLASH_ACE2_ATTR_R = crate::FieldReader;
#[doc = "Field `FLASH_ACE2_ATTR` writer - reg_flash_ace2_attr"]
pub type FLASH_ACE2_ATTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - reg_flash_ace2_attr"]
    #[inline(always)]
    pub fn flash_ace2_attr(&self) -> FLASH_ACE2_ATTR_R {
        FLASH_ACE2_ATTR_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE2_ATTR")
            .field(
                "flash_ace2_attr",
                &format_args!("{}", self.flash_ace2_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASH_ACE2_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_flash_ace2_attr"]
    #[inline(always)]
    #[must_use]
    pub fn flash_ace2_attr(&mut self) -> FLASH_ACE2_ATTR_W<FLASH_ACE2_ATTR_SPEC, 0> {
        FLASH_ACE2_ATTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB_CTRL_FLASH_ACE2_ATTR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE2_ATTR_SPEC;
impl crate::RegisterSpec for FLASH_ACE2_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ace2_attr::R`](R) reader structure"]
impl crate::Readable for FLASH_ACE2_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ace2_attr::W`](W) writer structure"]
impl crate::Writable for FLASH_ACE2_ATTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_ACE2_ATTR to value 0x03"]
impl crate::Resettable for FLASH_ACE2_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
