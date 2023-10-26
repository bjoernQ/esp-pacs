#[doc = "Register `DCACHE_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Register `DCACHE_PRELOCK_SCT_SIZE` writer"]
pub type W = crate::W<DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_SIZE` reader - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_SIZE` writer - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT1_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DCACHE_PRELOCK_SCT0_SIZE` reader - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `DCACHE_PRELOCK_SCT0_SIZE` writer - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT0_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn dcache_prelock_sct1_size(&self) -> DCACHE_PRELOCK_SCT1_SIZE_R {
        DCACHE_PRELOCK_SCT1_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn dcache_prelock_sct0_size(&self) -> DCACHE_PRELOCK_SCT0_SIZE_R {
        DCACHE_PRELOCK_SCT0_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOCK_SCT_SIZE")
            .field(
                "dcache_prelock_sct1_size",
                &format_args!("{}", self.dcache_prelock_sct1_size().bits()),
            )
            .field(
                "dcache_prelock_sct0_size",
                &format_args!("{}", self.dcache_prelock_sct0_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_PRELOCK_SCT_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_prelock_sct1_size(
        &mut self,
    ) -> DCACHE_PRELOCK_SCT1_SIZE_W<DCACHE_PRELOCK_SCT_SIZE_SPEC, 0> {
        DCACHE_PRELOCK_SCT1_SIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_prelock_sct0_size(
        &mut self,
    ) -> DCACHE_PRELOCK_SCT0_SIZE_W<DCACHE_PRELOCK_SCT_SIZE_SPEC, 16> {
        DCACHE_PRELOCK_SCT0_SIZE_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_prelock_sct_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_prelock_sct_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for DCACHE_PRELOCK_SCT_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_prelock_sct_size::W`](W) writer structure"]
impl crate::Writable for DCACHE_PRELOCK_SCT_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_PRELOCK_SCT_SIZE to value 0"]
impl crate::Resettable for DCACHE_PRELOCK_SCT_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
