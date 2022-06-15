#[doc = "Register `Core_0_STATUSTABLE1` reader"]
pub struct R(crate::R<CORE_0_STATUSTABLE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_STATUSTABLE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_STATUSTABLE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_STATUSTABLE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_0_STATUSTABLE1` writer"]
pub struct W(crate::W<CORE_0_STATUSTABLE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_STATUSTABLE1_SPEC>;
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
impl From<crate::W<CORE_0_STATUSTABLE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_STATUSTABLE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_FROM_WORLD_1` reader - This bit is used to confirm world before enter entry 1"]
pub type CORE_0_FROM_WORLD_1_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_FROM_WORLD_1` writer - This bit is used to confirm world before enter entry 1"]
pub type CORE_0_FROM_WORLD_1_W<'a> = crate::BitWriter<'a, u32, CORE_0_STATUSTABLE1_SPEC, bool, 0>;
#[doc = "Field `CORE_0_FROM_ENTRY_1` reader - This filed is used to confirm in which entry before enter entry 1"]
pub type CORE_0_FROM_ENTRY_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_FROM_ENTRY_1` writer - This filed is used to confirm in which entry before enter entry 1"]
pub type CORE_0_FROM_ENTRY_1_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_STATUSTABLE1_SPEC, u8, u8, 4, 1>;
#[doc = "Field `CORE_0_CURRENT_1` reader - This bit is used to confirm whether the current state is in entry 1"]
pub type CORE_0_CURRENT_1_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_CURRENT_1` writer - This bit is used to confirm whether the current state is in entry 1"]
pub type CORE_0_CURRENT_1_W<'a> = crate::BitWriter<'a, u32, CORE_0_STATUSTABLE1_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 1"]
    #[inline(always)]
    pub fn core_0_from_world_1(&self) -> CORE_0_FROM_WORLD_1_R {
        CORE_0_FROM_WORLD_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 1"]
    #[inline(always)]
    pub fn core_0_from_entry_1(&self) -> CORE_0_FROM_ENTRY_1_R {
        CORE_0_FROM_ENTRY_1_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 1"]
    #[inline(always)]
    pub fn core_0_current_1(&self) -> CORE_0_CURRENT_1_R {
        CORE_0_CURRENT_1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 1"]
    #[inline(always)]
    pub fn core_0_from_world_1(&mut self) -> CORE_0_FROM_WORLD_1_W {
        CORE_0_FROM_WORLD_1_W::new(self)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 1"]
    #[inline(always)]
    pub fn core_0_from_entry_1(&mut self) -> CORE_0_FROM_ENTRY_1_W {
        CORE_0_FROM_ENTRY_1_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 1"]
    #[inline(always)]
    pub fn core_0_current_1(&mut self) -> CORE_0_CURRENT_1_W {
        CORE_0_CURRENT_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register of world switch of entry 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_statustable1](index.html) module"]
pub struct CORE_0_STATUSTABLE1_SPEC;
impl crate::RegisterSpec for CORE_0_STATUSTABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_statustable1::R](R) reader structure"]
impl crate::Readable for CORE_0_STATUSTABLE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_statustable1::W](W) writer structure"]
impl crate::Writable for CORE_0_STATUSTABLE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_0_STATUSTABLE1 to value 0"]
impl crate::Resettable for CORE_0_STATUSTABLE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
