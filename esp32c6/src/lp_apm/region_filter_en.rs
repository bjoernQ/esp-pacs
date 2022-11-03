#[doc = "Register `REGION_FILTER_EN` reader"]
pub struct R(crate::R<REGION_FILTER_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION_FILTER_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION_FILTER_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION_FILTER_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION_FILTER_EN` writer"]
pub struct W(crate::W<REGION_FILTER_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION_FILTER_EN_SPEC>;
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
impl From<crate::W<REGION_FILTER_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION_FILTER_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION_FILTER_EN` reader - Region filter enable"]
pub type REGION_FILTER_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION_FILTER_EN` writer - Region filter enable"]
pub type REGION_FILTER_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REGION_FILTER_EN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Region filter enable"]
    #[inline(always)]
    pub fn region_filter_en(&self) -> REGION_FILTER_EN_R {
        REGION_FILTER_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Region filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn region_filter_en(&mut self) -> REGION_FILTER_EN_W<0> {
        REGION_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region filter enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_filter_en](index.html) module"]
pub struct REGION_FILTER_EN_SPEC;
impl crate::RegisterSpec for REGION_FILTER_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region_filter_en::R](R) reader structure"]
impl crate::Readable for REGION_FILTER_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region_filter_en::W](W) writer structure"]
impl crate::Writable for REGION_FILTER_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION_FILTER_EN to value 0x01"]
impl crate::Resettable for REGION_FILTER_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
