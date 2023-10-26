#[doc = "Register `L1_CACHE_OBJECT_CTRL` reader"]
pub type R = crate::R<L1_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Register `L1_CACHE_OBJECT_CTRL` writer"]
pub type W = crate::W<L1_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE0_TAG_OBJECT` reader - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_ICACHE0_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_TAG_OBJECT` reader - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_ICACHE1_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_TAG_OBJECT` reader - Reserved"]
pub type L1_ICACHE2_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_TAG_OBJECT` reader - Reserved"]
pub type L1_ICACHE3_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_CACHE_TAG_OBJECT` reader - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_CACHE_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_CACHE_TAG_OBJECT` writer - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_CACHE_TAG_OBJECT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L1_ICACHE0_MEM_OBJECT` reader - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_ICACHE0_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_MEM_OBJECT` reader - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_ICACHE1_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_MEM_OBJECT` reader - Reserved"]
pub type L1_ICACHE2_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_MEM_OBJECT` reader - Reserved"]
pub type L1_ICACHE3_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_CACHE_MEM_OBJECT` reader - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_CACHE_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `L1_CACHE_MEM_OBJECT` writer - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1_CACHE_MEM_OBJECT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache0_tag_object(&self) -> L1_ICACHE0_TAG_OBJECT_R {
        L1_ICACHE0_TAG_OBJECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache1_tag_object(&self) -> L1_ICACHE1_TAG_OBJECT_R {
        L1_ICACHE1_TAG_OBJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_object(&self) -> L1_ICACHE2_TAG_OBJECT_R {
        L1_ICACHE2_TAG_OBJECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_object(&self) -> L1_ICACHE3_TAG_OBJECT_R {
        L1_ICACHE3_TAG_OBJECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_cache_tag_object(&self) -> L1_CACHE_TAG_OBJECT_R {
        L1_CACHE_TAG_OBJECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache0_mem_object(&self) -> L1_ICACHE0_MEM_OBJECT_R {
        L1_ICACHE0_MEM_OBJECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache1_mem_object(&self) -> L1_ICACHE1_MEM_OBJECT_R {
        L1_ICACHE1_MEM_OBJECT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_mem_object(&self) -> L1_ICACHE2_MEM_OBJECT_R {
        L1_ICACHE2_MEM_OBJECT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_mem_object(&self) -> L1_ICACHE3_MEM_OBJECT_R {
        L1_ICACHE3_MEM_OBJECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_cache_mem_object(&self) -> L1_CACHE_MEM_OBJECT_R {
        L1_CACHE_MEM_OBJECT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_OBJECT_CTRL")
            .field(
                "l1_icache0_tag_object",
                &format_args!("{}", self.l1_icache0_tag_object().bit()),
            )
            .field(
                "l1_icache1_tag_object",
                &format_args!("{}", self.l1_icache1_tag_object().bit()),
            )
            .field(
                "l1_icache2_tag_object",
                &format_args!("{}", self.l1_icache2_tag_object().bit()),
            )
            .field(
                "l1_icache3_tag_object",
                &format_args!("{}", self.l1_icache3_tag_object().bit()),
            )
            .field(
                "l1_cache_tag_object",
                &format_args!("{}", self.l1_cache_tag_object().bit()),
            )
            .field(
                "l1_icache0_mem_object",
                &format_args!("{}", self.l1_icache0_mem_object().bit()),
            )
            .field(
                "l1_icache1_mem_object",
                &format_args!("{}", self.l1_icache1_mem_object().bit()),
            )
            .field(
                "l1_icache2_mem_object",
                &format_args!("{}", self.l1_icache2_mem_object().bit()),
            )
            .field(
                "l1_icache3_mem_object",
                &format_args!("{}", self.l1_icache3_mem_object().bit()),
            )
            .field(
                "l1_cache_mem_object",
                &format_args!("{}", self.l1_cache_mem_object().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_OBJECT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_tag_object(&mut self) -> L1_CACHE_TAG_OBJECT_W<L1_CACHE_OBJECT_CTRL_SPEC, 4> {
        L1_CACHE_TAG_OBJECT_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_mem_object(&mut self) -> L1_CACHE_MEM_OBJECT_W<L1_CACHE_OBJECT_CTRL_SPEC, 10> {
        L1_CACHE_MEM_OBJECT_W::new(self)
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
#[doc = "Cache Tag and Data memory Object control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_object_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_object_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_object_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_OBJECT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_object_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_OBJECT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_OBJECT_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_OBJECT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
