#[doc = "Register `LSCH1_DUTY` reader"]
pub struct R(crate::R<LSCH1_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH1_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH1_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH1_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH1_DUTY` writer"]
pub struct W(crate::W<LSCH1_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH1_DUTY_SPEC>;
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
impl From<crate::W<LSCH1_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH1_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_LSCH1` reader - reg_duty_lsch1."]
pub type DUTY_LSCH1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DUTY_LSCH1` writer - reg_duty_lsch1."]
pub type DUTY_LSCH1_W<'a> = crate::FieldWriter<'a, u32, LSCH1_DUTY_SPEC, u32, u32, 19, 0>;
impl R {
    #[doc = "Bits 0:18 - reg_duty_lsch1."]
    #[inline(always)]
    pub fn duty_lsch1(&self) -> DUTY_LSCH1_R {
        DUTY_LSCH1_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - reg_duty_lsch1."]
    #[inline(always)]
    pub fn duty_lsch1(&mut self) -> DUTY_LSCH1_W {
        DUTY_LSCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH1_DUTY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_duty](index.html) module"]
pub struct LSCH1_DUTY_SPEC;
impl crate::RegisterSpec for LSCH1_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch1_duty::R](R) reader structure"]
impl crate::Readable for LSCH1_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch1_duty::W](W) writer structure"]
impl crate::Writable for LSCH1_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH1_DUTY to value 0"]
impl crate::Resettable for LSCH1_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
