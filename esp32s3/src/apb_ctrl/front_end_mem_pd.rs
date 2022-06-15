#[doc = "Register `FRONT_END_MEM_PD` reader"]
pub struct R(crate::R<FRONT_END_MEM_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRONT_END_MEM_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRONT_END_MEM_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRONT_END_MEM_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRONT_END_MEM_PD` writer"]
pub struct W(crate::W<FRONT_END_MEM_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRONT_END_MEM_PD_SPEC>;
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
impl From<crate::W<FRONT_END_MEM_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRONT_END_MEM_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC_MEM_FORCE_PU` reader - ******* Description ***********"]
pub type AGC_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `AGC_MEM_FORCE_PU` writer - ******* Description ***********"]
pub type AGC_MEM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 0>;
#[doc = "Field `AGC_MEM_FORCE_PD` reader - ******* Description ***********"]
pub type AGC_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `AGC_MEM_FORCE_PD` writer - ******* Description ***********"]
pub type AGC_MEM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 1>;
#[doc = "Field `PBUS_MEM_FORCE_PU` reader - ******* Description ***********"]
pub type PBUS_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `PBUS_MEM_FORCE_PU` writer - ******* Description ***********"]
pub type PBUS_MEM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 2>;
#[doc = "Field `PBUS_MEM_FORCE_PD` reader - ******* Description ***********"]
pub type PBUS_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `PBUS_MEM_FORCE_PD` writer - ******* Description ***********"]
pub type PBUS_MEM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 3>;
#[doc = "Field `DC_MEM_FORCE_PU` reader - ******* Description ***********"]
pub type DC_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `DC_MEM_FORCE_PU` writer - ******* Description ***********"]
pub type DC_MEM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 4>;
#[doc = "Field `DC_MEM_FORCE_PD` reader - ******* Description ***********"]
pub type DC_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `DC_MEM_FORCE_PD` writer - ******* Description ***********"]
pub type DC_MEM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 5>;
#[doc = "Field `FREQ_MEM_FORCE_PU` reader - ******* Description ***********"]
pub type FREQ_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `FREQ_MEM_FORCE_PU` writer - ******* Description ***********"]
pub type FREQ_MEM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 6>;
#[doc = "Field `FREQ_MEM_FORCE_PD` reader - ******* Description ***********"]
pub type FREQ_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `FREQ_MEM_FORCE_PD` writer - ******* Description ***********"]
pub type FREQ_MEM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, FRONT_END_MEM_PD_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - ******* Description ***********"]
    #[inline(always)]
    pub fn agc_mem_force_pu(&self) -> AGC_MEM_FORCE_PU_R {
        AGC_MEM_FORCE_PU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ******* Description ***********"]
    #[inline(always)]
    pub fn agc_mem_force_pd(&self) -> AGC_MEM_FORCE_PD_R {
        AGC_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ******* Description ***********"]
    #[inline(always)]
    pub fn pbus_mem_force_pu(&self) -> PBUS_MEM_FORCE_PU_R {
        PBUS_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ******* Description ***********"]
    #[inline(always)]
    pub fn pbus_mem_force_pd(&self) -> PBUS_MEM_FORCE_PD_R {
        PBUS_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ******* Description ***********"]
    #[inline(always)]
    pub fn dc_mem_force_pu(&self) -> DC_MEM_FORCE_PU_R {
        DC_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ******* Description ***********"]
    #[inline(always)]
    pub fn dc_mem_force_pd(&self) -> DC_MEM_FORCE_PD_R {
        DC_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ******* Description ***********"]
    #[inline(always)]
    pub fn freq_mem_force_pu(&self) -> FREQ_MEM_FORCE_PU_R {
        FREQ_MEM_FORCE_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ******* Description ***********"]
    #[inline(always)]
    pub fn freq_mem_force_pd(&self) -> FREQ_MEM_FORCE_PD_R {
        FREQ_MEM_FORCE_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ******* Description ***********"]
    #[inline(always)]
    pub fn agc_mem_force_pu(&mut self) -> AGC_MEM_FORCE_PU_W {
        AGC_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 1 - ******* Description ***********"]
    #[inline(always)]
    pub fn agc_mem_force_pd(&mut self) -> AGC_MEM_FORCE_PD_W {
        AGC_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 2 - ******* Description ***********"]
    #[inline(always)]
    pub fn pbus_mem_force_pu(&mut self) -> PBUS_MEM_FORCE_PU_W {
        PBUS_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 3 - ******* Description ***********"]
    #[inline(always)]
    pub fn pbus_mem_force_pd(&mut self) -> PBUS_MEM_FORCE_PD_W {
        PBUS_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 4 - ******* Description ***********"]
    #[inline(always)]
    pub fn dc_mem_force_pu(&mut self) -> DC_MEM_FORCE_PU_W {
        DC_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 5 - ******* Description ***********"]
    #[inline(always)]
    pub fn dc_mem_force_pd(&mut self) -> DC_MEM_FORCE_PD_W {
        DC_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 6 - ******* Description ***********"]
    #[inline(always)]
    pub fn freq_mem_force_pu(&mut self) -> FREQ_MEM_FORCE_PU_W {
        FREQ_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 7 - ******* Description ***********"]
    #[inline(always)]
    pub fn freq_mem_force_pd(&mut self) -> FREQ_MEM_FORCE_PD_W {
        FREQ_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [front_end_mem_pd](index.html) module"]
pub struct FRONT_END_MEM_PD_SPEC;
impl crate::RegisterSpec for FRONT_END_MEM_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [front_end_mem_pd::R](R) reader structure"]
impl crate::Readable for FRONT_END_MEM_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [front_end_mem_pd::W](W) writer structure"]
impl crate::Writable for FRONT_END_MEM_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRONT_END_MEM_PD to value 0x55"]
impl crate::Resettable for FRONT_END_MEM_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55
    }
}
