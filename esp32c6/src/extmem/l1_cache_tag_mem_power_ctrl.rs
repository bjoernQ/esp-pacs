#[doc = "Register `L1_CACHE_TAG_MEM_POWER_CTRL` reader"]
pub struct R(crate::R<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_TAG_MEM_POWER_CTRL` writer"]
pub struct W(crate::W<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-ICache0 tag memory. 1: close gating, 0: open clock gating."]
pub type L1_ICACHE0_TAG_MEM_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_PD` reader - The bit is used to power L1-ICache0 tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_ICACHE0_TAG_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_PU` reader - The bit is used to power L1-ICache0 tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_ICACHE0_TAG_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-ICache1 tag memory. 1: close gating, 0: open clock gating."]
pub type L1_ICACHE1_TAG_MEM_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_PD` reader - The bit is used to power L1-ICache1 tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_ICACHE1_TAG_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_PU` reader - The bit is used to power L1-ICache1 tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_ICACHE1_TAG_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE2_TAG_MEM_FORCE_ON` reader - Reserved"]
pub type L1_ICACHE2_TAG_MEM_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE2_TAG_MEM_FORCE_PD` reader - Reserved"]
pub type L1_ICACHE2_TAG_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE2_TAG_MEM_FORCE_PU` reader - Reserved"]
pub type L1_ICACHE2_TAG_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE3_TAG_MEM_FORCE_ON` reader - Reserved"]
pub type L1_ICACHE3_TAG_MEM_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE3_TAG_MEM_FORCE_PD` reader - Reserved"]
pub type L1_ICACHE3_TAG_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE3_TAG_MEM_FORCE_PU` reader - Reserved"]
pub type L1_ICACHE3_TAG_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-Cache tag memory. 1: close gating, 0: open clock gating."]
pub type L1_CACHE_TAG_MEM_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_TAG_MEM_FORCE_ON` writer - The bit is used to close clock gating of L1-Cache tag memory. 1: close gating, 0: open clock gating."]
pub type L1_CACHE_TAG_MEM_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_TAG_MEM_POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_CACHE_TAG_MEM_FORCE_PD` reader - The bit is used to power L1-Cache tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_CACHE_TAG_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_TAG_MEM_FORCE_PD` writer - The bit is used to power L1-Cache tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_CACHE_TAG_MEM_FORCE_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_TAG_MEM_POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_CACHE_TAG_MEM_FORCE_PU` reader - The bit is used to power L1-Cache tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_CACHE_TAG_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_TAG_MEM_FORCE_PU` writer - The bit is used to power L1-Cache tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_CACHE_TAG_MEM_FORCE_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_TAG_MEM_POWER_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to close clock gating of L1-ICache0 tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_on(&self) -> L1_ICACHE0_TAG_MEM_FORCE_ON_R {
        L1_ICACHE0_TAG_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power L1-ICache0 tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_pd(&self) -> L1_ICACHE0_TAG_MEM_FORCE_PD_R {
        L1_ICACHE0_TAG_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power L1-ICache0 tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_pu(&self) -> L1_ICACHE0_TAG_MEM_FORCE_PU_R {
        L1_ICACHE0_TAG_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to close clock gating of L1-ICache1 tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_on(&self) -> L1_ICACHE1_TAG_MEM_FORCE_ON_R {
        L1_ICACHE1_TAG_MEM_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to power L1-ICache1 tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_pd(&self) -> L1_ICACHE1_TAG_MEM_FORCE_PD_R {
        L1_ICACHE1_TAG_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to power L1-ICache1 tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_pu(&self) -> L1_ICACHE1_TAG_MEM_FORCE_PU_R {
        L1_ICACHE1_TAG_MEM_FORCE_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_mem_force_on(&self) -> L1_ICACHE2_TAG_MEM_FORCE_ON_R {
        L1_ICACHE2_TAG_MEM_FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_mem_force_pd(&self) -> L1_ICACHE2_TAG_MEM_FORCE_PD_R {
        L1_ICACHE2_TAG_MEM_FORCE_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_mem_force_pu(&self) -> L1_ICACHE2_TAG_MEM_FORCE_PU_R {
        L1_ICACHE2_TAG_MEM_FORCE_PU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_mem_force_on(&self) -> L1_ICACHE3_TAG_MEM_FORCE_ON_R {
        L1_ICACHE3_TAG_MEM_FORCE_ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_mem_force_pd(&self) -> L1_ICACHE3_TAG_MEM_FORCE_PD_R {
        L1_ICACHE3_TAG_MEM_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_mem_force_pu(&self) -> L1_ICACHE3_TAG_MEM_FORCE_PU_R {
        L1_ICACHE3_TAG_MEM_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to close clock gating of L1-Cache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_cache_tag_mem_force_on(&self) -> L1_CACHE_TAG_MEM_FORCE_ON_R {
        L1_CACHE_TAG_MEM_FORCE_ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to power L1-Cache tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_cache_tag_mem_force_pd(&self) -> L1_CACHE_TAG_MEM_FORCE_PD_R {
        L1_CACHE_TAG_MEM_FORCE_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to power L1-Cache tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_cache_tag_mem_force_pu(&self) -> L1_CACHE_TAG_MEM_FORCE_PU_R {
        L1_CACHE_TAG_MEM_FORCE_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - The bit is used to close clock gating of L1-Cache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_tag_mem_force_on(&mut self) -> L1_CACHE_TAG_MEM_FORCE_ON_W<16> {
        L1_CACHE_TAG_MEM_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 17 - The bit is used to power L1-Cache tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_tag_mem_force_pd(&mut self) -> L1_CACHE_TAG_MEM_FORCE_PD_W<17> {
        L1_CACHE_TAG_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 18 - The bit is used to power L1-Cache tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_tag_mem_force_pu(&mut self) -> L1_CACHE_TAG_MEM_FORCE_PU_W<18> {
        L1_CACHE_TAG_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache tag memory power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_tag_mem_power_ctrl](index.html) module"]
pub struct L1_CACHE_TAG_MEM_POWER_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_TAG_MEM_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_tag_mem_power_ctrl::R](R) reader structure"]
impl crate::Readable for L1_CACHE_TAG_MEM_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_tag_mem_power_ctrl::W](W) writer structure"]
impl crate::Writable for L1_CACHE_TAG_MEM_POWER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_TAG_MEM_POWER_CTRL to value 0x0005_5555"]
impl crate::Resettable for L1_CACHE_TAG_MEM_POWER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_5555;
}
