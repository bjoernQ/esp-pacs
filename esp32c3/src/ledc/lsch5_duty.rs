#[doc = "Register `LSCH5_DUTY` reader"]
pub struct R(crate::R<LSCH5_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH5_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH5_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH5_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH5_DUTY` writer"]
pub struct W(crate::W<LSCH5_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH5_DUTY_SPEC>;
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
impl From<crate::W<LSCH5_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH5_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_LSCH5` reader - reg_duty_lsch5."]
pub type DUTY_LSCH5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DUTY_LSCH5` writer - reg_duty_lsch5."]
pub type DUTY_LSCH5_W<'a> = crate::FieldWriter<'a, u32, LSCH5_DUTY_SPEC, u32, u32, 19, 0>;
impl R {
    #[doc = "Bits 0:18 - reg_duty_lsch5."]
    #[inline(always)]
    pub fn duty_lsch5(&self) -> DUTY_LSCH5_R {
        DUTY_LSCH5_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - reg_duty_lsch5."]
    #[inline(always)]
    pub fn duty_lsch5(&mut self) -> DUTY_LSCH5_W {
        DUTY_LSCH5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH5_DUTY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_duty](index.html) module"]
pub struct LSCH5_DUTY_SPEC;
impl crate::RegisterSpec for LSCH5_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch5_duty::R](R) reader structure"]
impl crate::Readable for LSCH5_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch5_duty::W](W) writer structure"]
impl crate::Writable for LSCH5_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH5_DUTY to value 0"]
impl crate::Resettable for LSCH5_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
