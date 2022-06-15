#[doc = "Register `CARRIER1_CFG` reader"]
pub struct R(crate::R<CARRIER1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CARRIER1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CARRIER1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CARRIER1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CARRIER1_CFG` writer"]
pub struct W(crate::W<CARRIER1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CARRIER1_CFG_SPEC>;
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
impl From<crate::W<CARRIER1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CARRIER1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER1_EN` reader - "]
pub type CARRIER1_EN_R = crate::BitReader<bool>;
#[doc = "Field `CARRIER1_EN` writer - "]
pub type CARRIER1_EN_W<'a> = crate::BitWriter<'a, u32, CARRIER1_CFG_SPEC, bool, 0>;
#[doc = "Field `CARRIER1_PRESCALE` reader - "]
pub type CARRIER1_PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CARRIER1_PRESCALE` writer - "]
pub type CARRIER1_PRESCALE_W<'a> = crate::FieldWriter<'a, u32, CARRIER1_CFG_SPEC, u8, u8, 4, 1>;
#[doc = "Field `CARRIER1_DUTY` reader - "]
pub type CARRIER1_DUTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CARRIER1_DUTY` writer - "]
pub type CARRIER1_DUTY_W<'a> = crate::FieldWriter<'a, u32, CARRIER1_CFG_SPEC, u8, u8, 3, 5>;
#[doc = "Field `CARRIER1_OSHTWTH` reader - "]
pub type CARRIER1_OSHTWTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CARRIER1_OSHTWTH` writer - "]
pub type CARRIER1_OSHTWTH_W<'a> = crate::FieldWriter<'a, u32, CARRIER1_CFG_SPEC, u8, u8, 4, 8>;
#[doc = "Field `CARRIER1_OUT_INVERT` reader - "]
pub type CARRIER1_OUT_INVERT_R = crate::BitReader<bool>;
#[doc = "Field `CARRIER1_OUT_INVERT` writer - "]
pub type CARRIER1_OUT_INVERT_W<'a> = crate::BitWriter<'a, u32, CARRIER1_CFG_SPEC, bool, 12>;
#[doc = "Field `CARRIER1_IN_INVERT` reader - "]
pub type CARRIER1_IN_INVERT_R = crate::BitReader<bool>;
#[doc = "Field `CARRIER1_IN_INVERT` writer - "]
pub type CARRIER1_IN_INVERT_W<'a> = crate::BitWriter<'a, u32, CARRIER1_CFG_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn carrier1_en(&self) -> CARRIER1_EN_R {
        CARRIER1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn carrier1_prescale(&self) -> CARRIER1_PRESCALE_R {
        CARRIER1_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn carrier1_duty(&self) -> CARRIER1_DUTY_R {
        CARRIER1_DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn carrier1_oshtwth(&self) -> CARRIER1_OSHTWTH_R {
        CARRIER1_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn carrier1_out_invert(&self) -> CARRIER1_OUT_INVERT_R {
        CARRIER1_OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn carrier1_in_invert(&self) -> CARRIER1_IN_INVERT_R {
        CARRIER1_IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn carrier1_en(&mut self) -> CARRIER1_EN_W {
        CARRIER1_EN_W::new(self)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn carrier1_prescale(&mut self) -> CARRIER1_PRESCALE_W {
        CARRIER1_PRESCALE_W::new(self)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn carrier1_duty(&mut self) -> CARRIER1_DUTY_W {
        CARRIER1_DUTY_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn carrier1_oshtwth(&mut self) -> CARRIER1_OSHTWTH_W {
        CARRIER1_OSHTWTH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn carrier1_out_invert(&mut self) -> CARRIER1_OUT_INVERT_W {
        CARRIER1_OUT_INVERT_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn carrier1_in_invert(&mut self) -> CARRIER1_IN_INVERT_W {
        CARRIER1_IN_INVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [carrier1_cfg](index.html) module"]
pub struct CARRIER1_CFG_SPEC;
impl crate::RegisterSpec for CARRIER1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [carrier1_cfg::R](R) reader structure"]
impl crate::Readable for CARRIER1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [carrier1_cfg::W](W) writer structure"]
impl crate::Writable for CARRIER1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CARRIER1_CFG to value 0"]
impl crate::Resettable for CARRIER1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
