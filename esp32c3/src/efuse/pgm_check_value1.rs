#[doc = "Register `PGM_CHECK_VALUE1` reader"]
pub struct R(crate::R<PGM_CHECK_VALUE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_CHECK_VALUE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_CHECK_VALUE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_CHECK_VALUE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_CHECK_VALUE1` writer"]
pub struct W(crate::W<PGM_CHECK_VALUE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_CHECK_VALUE1_SPEC>;
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
impl From<crate::W<PGM_CHECK_VALUE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_CHECK_VALUE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGM_RS_DATA_1` reader - The content of the 1st 32-bit RS code to be programmed."]
pub struct PGM_RS_DATA_1_R(crate::FieldReader<u32, u32>);
impl PGM_RS_DATA_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PGM_RS_DATA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_RS_DATA_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_RS_DATA_1` writer - The content of the 1st 32-bit RS code to be programmed."]
pub struct PGM_RS_DATA_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_RS_DATA_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of the 1st 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data_1(&self) -> PGM_RS_DATA_1_R {
        PGM_RS_DATA_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the 1st 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data_1(&mut self) -> PGM_RS_DATA_1_W {
        PGM_RS_DATA_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register 1 that stores the RS code to be programmed.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_check_value1]
(index.html) module"]
pub struct PGM_CHECK_VALUE1_SPEC;
impl crate::RegisterSpec for PGM_CHECK_VALUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_check_value1::R]
(R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_check_value1::W]
(W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGM_CHECK_VALUE1 to value 0"]
impl crate::Resettable for PGM_CHECK_VALUE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}