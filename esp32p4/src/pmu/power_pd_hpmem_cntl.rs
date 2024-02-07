#[doc = "Register `POWER_PD_HPMEM_CNTL` reader"]
pub type R = crate::R<POWER_PD_HPMEM_CNTL_SPEC>;
#[doc = "Register `POWER_PD_HPMEM_CNTL` writer"]
pub type W = crate::W<POWER_PD_HPMEM_CNTL_SPEC>;
#[doc = "Field `FORCE_HP_MEM_RESET` reader - need_des"]
pub type FORCE_HP_MEM_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_HP_MEM_RESET` writer - need_des"]
pub type FORCE_HP_MEM_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_MEM_ISO` reader - need_des"]
pub type FORCE_HP_MEM_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_HP_MEM_ISO` writer - need_des"]
pub type FORCE_HP_MEM_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_MEM_PU` reader - need_des"]
pub type FORCE_HP_MEM_PU_R = crate::BitReader;
#[doc = "Field `FORCE_HP_MEM_PU` writer - need_des"]
pub type FORCE_HP_MEM_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_MEM_NO_RESET` reader - need_des"]
pub type FORCE_HP_MEM_NO_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_HP_MEM_NO_RESET` writer - need_des"]
pub type FORCE_HP_MEM_NO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_MEM_NO_ISO` reader - need_des"]
pub type FORCE_HP_MEM_NO_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_HP_MEM_NO_ISO` writer - need_des"]
pub type FORCE_HP_MEM_NO_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_MEM_PD` reader - need_des"]
pub type FORCE_HP_MEM_PD_R = crate::BitReader;
#[doc = "Field `FORCE_HP_MEM_PD` writer - need_des"]
pub type FORCE_HP_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_reset(&self) -> FORCE_HP_MEM_RESET_R {
        FORCE_HP_MEM_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_iso(&self) -> FORCE_HP_MEM_ISO_R {
        FORCE_HP_MEM_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_pu(&self) -> FORCE_HP_MEM_PU_R {
        FORCE_HP_MEM_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_no_reset(&self) -> FORCE_HP_MEM_NO_RESET_R {
        FORCE_HP_MEM_NO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_no_iso(&self) -> FORCE_HP_MEM_NO_ISO_R {
        FORCE_HP_MEM_NO_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_pd(&self) -> FORCE_HP_MEM_PD_R {
        FORCE_HP_MEM_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_HPMEM_CNTL")
            .field(
                "force_hp_mem_reset",
                &format_args!("{}", self.force_hp_mem_reset().bit()),
            )
            .field(
                "force_hp_mem_iso",
                &format_args!("{}", self.force_hp_mem_iso().bit()),
            )
            .field(
                "force_hp_mem_pu",
                &format_args!("{}", self.force_hp_mem_pu().bit()),
            )
            .field(
                "force_hp_mem_no_reset",
                &format_args!("{}", self.force_hp_mem_no_reset().bit()),
            )
            .field(
                "force_hp_mem_no_iso",
                &format_args!("{}", self.force_hp_mem_no_iso().bit()),
            )
            .field(
                "force_hp_mem_pd",
                &format_args!("{}", self.force_hp_mem_pd().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_PD_HPMEM_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_reset(&mut self) -> FORCE_HP_MEM_RESET_W<POWER_PD_HPMEM_CNTL_SPEC> {
        FORCE_HP_MEM_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_iso(&mut self) -> FORCE_HP_MEM_ISO_W<POWER_PD_HPMEM_CNTL_SPEC> {
        FORCE_HP_MEM_ISO_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_pu(&mut self) -> FORCE_HP_MEM_PU_W<POWER_PD_HPMEM_CNTL_SPEC> {
        FORCE_HP_MEM_PU_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_no_reset(&mut self) -> FORCE_HP_MEM_NO_RESET_W<POWER_PD_HPMEM_CNTL_SPEC> {
        FORCE_HP_MEM_NO_RESET_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_no_iso(&mut self) -> FORCE_HP_MEM_NO_ISO_W<POWER_PD_HPMEM_CNTL_SPEC> {
        FORCE_HP_MEM_NO_ISO_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_pd(&mut self) -> FORCE_HP_MEM_PD_W<POWER_PD_HPMEM_CNTL_SPEC> {
        FORCE_HP_MEM_PD_W::new(self, 5)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_pd_hpmem_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_hpmem_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_PD_HPMEM_CNTL_SPEC;
impl crate::RegisterSpec for POWER_PD_HPMEM_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_hpmem_cntl::R`](R) reader structure"]
impl crate::Readable for POWER_PD_HPMEM_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_pd_hpmem_cntl::W`](W) writer structure"]
impl crate::Writable for POWER_PD_HPMEM_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_PD_HPMEM_CNTL to value 0x1c"]
impl crate::Resettable for POWER_PD_HPMEM_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x1c;
}