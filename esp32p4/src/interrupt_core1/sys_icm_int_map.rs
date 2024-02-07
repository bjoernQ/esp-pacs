#[doc = "Register `SYS_ICM_INT_MAP` reader"]
pub type R = crate::R<SYS_ICM_INT_MAP_SPEC>;
#[doc = "Register `SYS_ICM_INT_MAP` writer"]
pub type W = crate::W<SYS_ICM_INT_MAP_SPEC>;
#[doc = "Field `CORE1_SYS_ICM_INT_MAP` reader - NA"]
pub type CORE1_SYS_ICM_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE1_SYS_ICM_INT_MAP` writer - NA"]
pub type CORE1_SYS_ICM_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core1_sys_icm_int_map(&self) -> CORE1_SYS_ICM_INT_MAP_R {
        CORE1_SYS_ICM_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_ICM_INT_MAP")
            .field(
                "core1_sys_icm_int_map",
                &format_args!("{}", self.core1_sys_icm_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYS_ICM_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn core1_sys_icm_int_map(&mut self) -> CORE1_SYS_ICM_INT_MAP_W<SYS_ICM_INT_MAP_SPEC> {
        CORE1_SYS_ICM_INT_MAP_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_icm_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_icm_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_ICM_INT_MAP_SPEC;
impl crate::RegisterSpec for SYS_ICM_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_icm_int_map::R`](R) reader structure"]
impl crate::Readable for SYS_ICM_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_icm_int_map::W`](W) writer structure"]
impl crate::Writable for SYS_ICM_INT_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_ICM_INT_MAP to value 0"]
impl crate::Resettable for SYS_ICM_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}