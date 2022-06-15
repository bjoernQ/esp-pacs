#[doc = "Register `TEXT_OUT_2` reader"]
pub struct R(crate::R<TEXT_OUT_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEXT_OUT_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEXT_OUT_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEXT_OUT_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEXT_OUT_2` writer"]
pub struct W(crate::W<TEXT_OUT_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXT_OUT_2_SPEC>;
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
impl From<crate::W<TEXT_OUT_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXT_OUT_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXT_OUT_2` reader - This bits stores text_out_2 that is a part of result text material."]
pub type TEXT_OUT_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TEXT_OUT_2` writer - This bits stores text_out_2 that is a part of result text material."]
pub type TEXT_OUT_2_W<'a> = crate::FieldWriter<'a, u32, TEXT_OUT_2_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This bits stores text_out_2 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_2(&self) -> TEXT_OUT_2_R {
        TEXT_OUT_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores text_out_2 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_2(&mut self) -> TEXT_OUT_2_W {
        TEXT_OUT_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "result text material text_out_2 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [text_out_2](index.html) module"]
pub struct TEXT_OUT_2_SPEC;
impl crate::RegisterSpec for TEXT_OUT_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [text_out_2::R](R) reader structure"]
impl crate::Readable for TEXT_OUT_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [text_out_2::W](W) writer structure"]
impl crate::Writable for TEXT_OUT_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEXT_OUT_2 to value 0"]
impl crate::Resettable for TEXT_OUT_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
