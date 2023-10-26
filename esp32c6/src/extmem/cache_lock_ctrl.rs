#[doc = "Register `CACHE_LOCK_CTRL` reader"]
pub type R = crate::R<CACHE_LOCK_CTRL_SPEC>;
#[doc = "Register `CACHE_LOCK_CTRL` writer"]
pub type W = crate::W<CACHE_LOCK_CTRL_SPEC>;
#[doc = "Field `CACHE_LOCK_ENA` reader - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done"]
pub type CACHE_LOCK_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_LOCK_ENA` writer - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done"]
pub type CACHE_LOCK_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CACHE_UNLOCK_ENA` reader - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done"]
pub type CACHE_UNLOCK_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_UNLOCK_ENA` writer - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done"]
pub type CACHE_UNLOCK_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CACHE_LOCK_DONE` reader - The bit is used to indicate whether unlock/lock operation is finished or not. 0: not finished. 1: finished."]
pub type CACHE_LOCK_DONE_R = crate::BitReader;
#[doc = "Field `CACHE_LOCK_RGID` reader - The bit is used to set the gid of cache lock/unlock."]
pub type CACHE_LOCK_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done"]
    #[inline(always)]
    pub fn cache_lock_ena(&self) -> CACHE_LOCK_ENA_R {
        CACHE_LOCK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done"]
    #[inline(always)]
    pub fn cache_unlock_ena(&self) -> CACHE_UNLOCK_ENA_R {
        CACHE_UNLOCK_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate whether unlock/lock operation is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn cache_lock_done(&self) -> CACHE_LOCK_DONE_R {
        CACHE_LOCK_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of cache lock/unlock."]
    #[inline(always)]
    pub fn cache_lock_rgid(&self) -> CACHE_LOCK_RGID_R {
        CACHE_LOCK_RGID_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_LOCK_CTRL")
            .field(
                "cache_lock_ena",
                &format_args!("{}", self.cache_lock_ena().bit()),
            )
            .field(
                "cache_unlock_ena",
                &format_args!("{}", self.cache_unlock_ena().bit()),
            )
            .field(
                "cache_lock_done",
                &format_args!("{}", self.cache_lock_done().bit()),
            )
            .field(
                "cache_lock_rgid",
                &format_args!("{}", self.cache_lock_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_LOCK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done"]
    #[inline(always)]
    #[must_use]
    pub fn cache_lock_ena(&mut self) -> CACHE_LOCK_ENA_W<CACHE_LOCK_CTRL_SPEC, 0> {
        CACHE_LOCK_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done"]
    #[inline(always)]
    #[must_use]
    pub fn cache_unlock_ena(&mut self) -> CACHE_UNLOCK_ENA_W<CACHE_LOCK_CTRL_SPEC, 1> {
        CACHE_UNLOCK_ENA_W::new(self)
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
#[doc = "Lock-class (manual lock) operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_lock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_lock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_LOCK_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_LOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_lock_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_LOCK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_lock_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_LOCK_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_LOCK_CTRL to value 0x04"]
impl crate::Resettable for CACHE_LOCK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
