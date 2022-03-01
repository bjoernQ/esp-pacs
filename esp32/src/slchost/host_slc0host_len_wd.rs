#[doc = "Register `HOST_SLC0HOST_LEN_WD` reader"]
pub struct R(crate::R<HOST_SLC0HOST_LEN_WD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC0HOST_LEN_WD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC0HOST_LEN_WD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC0HOST_LEN_WD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLC0HOST_LEN_WD` writer"]
pub struct W(crate::W<HOST_SLC0HOST_LEN_WD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC0HOST_LEN_WD_SPEC>;
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
impl From<crate::W<HOST_SLC0HOST_LEN_WD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC0HOST_LEN_WD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC0HOST_LEN_WD` reader - "]
pub struct HOST_SLC0HOST_LEN_WD_R(crate::FieldReader<u32, u32>);
impl HOST_SLC0HOST_LEN_WD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HOST_SLC0HOST_LEN_WD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0HOST_LEN_WD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0HOST_LEN_WD` writer - "]
pub struct HOST_SLC0HOST_LEN_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_LEN_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc0host_len_wd(&self) -> HOST_SLC0HOST_LEN_WD_R {
        HOST_SLC0HOST_LEN_WD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc0host_len_wd(&mut self) -> HOST_SLC0HOST_LEN_WD_W {
        HOST_SLC0HOST_LEN_WD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_len_wd]
(index.html) module"]
pub struct HOST_SLC0HOST_LEN_WD_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_LEN_WD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc0host_len_wd::R]
(R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_LEN_WD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slc0host_len_wd::W]
(W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_LEN_WD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_LEN_WD to value 0"]
impl crate::Resettable for HOST_SLC0HOST_LEN_WD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}