#[doc = "Register `Core_1_World_IRam0` reader"]
pub struct R(crate::R<CORE_1_WORLD_IRAM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_WORLD_IRAM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_WORLD_IRAM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_WORLD_IRAM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_World_IRam0` writer"]
pub struct W(crate::W<CORE_1_WORLD_IRAM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_WORLD_IRAM0_SPEC>;
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
impl From<crate::W<CORE_1_WORLD_IRAM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_WORLD_IRAM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_WORLD_IRAM0` reader - this field is used to read current world of Iram0 bus"]
pub type CORE_1_WORLD_IRAM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_WORLD_IRAM0` writer - this field is used to read current world of Iram0 bus"]
pub type CORE_1_WORLD_IRAM0_W<'a> =
    crate::FieldWriter<'a, u32, CORE_1_WORLD_IRAM0_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    pub fn core_1_world_iram0(&self) -> CORE_1_WORLD_IRAM0_R {
        CORE_1_WORLD_IRAM0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    pub fn core_1_world_iram0(&mut self) -> CORE_1_WORLD_IRAM0_W {
        CORE_1_WORLD_IRAM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_1 Iram0 world register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_world_iram0](index.html) module"]
pub struct CORE_1_WORLD_IRAM0_SPEC;
impl crate::RegisterSpec for CORE_1_WORLD_IRAM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_world_iram0::R](R) reader structure"]
impl crate::Readable for CORE_1_WORLD_IRAM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_world_iram0::W](W) writer structure"]
impl crate::Writable for CORE_1_WORLD_IRAM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_1_World_IRam0 to value 0"]
impl crate::Resettable for CORE_1_WORLD_IRAM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
