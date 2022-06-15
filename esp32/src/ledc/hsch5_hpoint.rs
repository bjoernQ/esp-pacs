#[doc = "Register `HSCH5_HPOINT` reader"]
pub struct R(crate::R<HSCH5_HPOINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH5_HPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH5_HPOINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH5_HPOINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH5_HPOINT` writer"]
pub struct W(crate::W<HSCH5_HPOINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH5_HPOINT_SPEC>;
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
impl From<crate::W<HSCH5_HPOINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH5_HPOINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOINT_HSCH5` reader - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel5 has reached reg_hpoint_hsch5\\[19:0\\]"]
pub type HPOINT_HSCH5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HPOINT_HSCH5` writer - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel5 has reached reg_hpoint_hsch5\\[19:0\\]"]
pub type HPOINT_HSCH5_W<'a> = crate::FieldWriter<'a, u32, HSCH5_HPOINT_SPEC, u32, u32, 20, 0>;
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel5 has reached reg_hpoint_hsch5\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch5(&self) -> HPOINT_HSCH5_R {
        HPOINT_HSCH5_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel5 has reached reg_hpoint_hsch5\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch5(&mut self) -> HPOINT_HSCH5_W {
        HPOINT_HSCH5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch5_hpoint](index.html) module"]
pub struct HSCH5_HPOINT_SPEC;
impl crate::RegisterSpec for HSCH5_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch5_hpoint::R](R) reader structure"]
impl crate::Readable for HSCH5_HPOINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch5_hpoint::W](W) writer structure"]
impl crate::Writable for HSCH5_HPOINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH5_HPOINT to value 0"]
impl crate::Resettable for HSCH5_HPOINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
