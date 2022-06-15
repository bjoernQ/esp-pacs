#[doc = "Register `PRO_TRACE_1` reader"]
pub struct R(crate::R<PRO_TRACE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_TRACE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_TRACE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_TRACE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_TRACE_1` writer"]
pub struct W(crate::W<PRO_TRACE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_TRACE_1_SPEC>;
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
impl From<crate::W<PRO_TRACE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_TRACE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_TRACE_DISABLE` reader - Setting to 1 disables the trace memory function."]
pub type PRO_TRACE_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `PRO_TRACE_DISABLE` writer - Setting to 1 disables the trace memory function."]
pub type PRO_TRACE_DISABLE_W<'a> = crate::BitWriter<'a, u32, PRO_TRACE_1_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Setting to 1 disables the trace memory function."]
    #[inline(always)]
    pub fn pro_trace_disable(&self) -> PRO_TRACE_DISABLE_R {
        PRO_TRACE_DISABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 disables the trace memory function."]
    #[inline(always)]
    pub fn pro_trace_disable(&mut self) -> PRO_TRACE_DISABLE_W {
        PRO_TRACE_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace memory permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_trace_1](index.html) module"]
pub struct PRO_TRACE_1_SPEC;
impl crate::RegisterSpec for PRO_TRACE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_trace_1::R](R) reader structure"]
impl crate::Readable for PRO_TRACE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_trace_1::W](W) writer structure"]
impl crate::Writable for PRO_TRACE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_TRACE_1 to value 0"]
impl crate::Resettable for PRO_TRACE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
