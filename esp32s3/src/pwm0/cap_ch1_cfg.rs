#[doc = "Register `CAP_CH1_CFG` reader"]
pub struct R(crate::R<CAP_CH1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_CH1_CFG` writer"]
pub struct W(crate::W<CAP_CH1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CH1_CFG_SPEC>;
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
impl From<crate::W<CAP_CH1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CH1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP1_EN` reader - When set, capture on channel 2 is enabled"]
pub type CAP1_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAP1_EN` writer - When set, capture on channel 2 is enabled"]
pub type CAP1_EN_W<'a> = crate::BitWriter<'a, u32, CAP_CH1_CFG_SPEC, bool, 0>;
#[doc = "Field `CAP1_MODE` reader - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP1_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP1_MODE` writer - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP1_MODE_W<'a> = crate::FieldWriter<'a, u32, CAP_CH1_CFG_SPEC, u8, u8, 2, 1>;
#[doc = "Field `CAP1_PRESCALE` reader - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
pub type CAP1_PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP1_PRESCALE` writer - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
pub type CAP1_PRESCALE_W<'a> = crate::FieldWriter<'a, u32, CAP_CH1_CFG_SPEC, u8, u8, 8, 3>;
#[doc = "Field `CAP1_IN_INVERT` reader - when set, CAP1 form GPIO matrix is inverted before prescale"]
pub type CAP1_IN_INVERT_R = crate::BitReader<bool>;
#[doc = "Field `CAP1_IN_INVERT` writer - when set, CAP1 form GPIO matrix is inverted before prescale"]
pub type CAP1_IN_INVERT_W<'a> = crate::BitWriter<'a, u32, CAP_CH1_CFG_SPEC, bool, 11>;
#[doc = "Field `CAP1_SW` writer - Write 1 will trigger a software forced capture on channel 1"]
pub type CAP1_SW_W<'a> = crate::BitWriter<'a, u32, CAP_CH1_CFG_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap1_en(&self) -> CAP1_EN_R {
        CAP1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap1_mode(&self) -> CAP1_MODE_R {
        CAP1_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap1_prescale(&self) -> CAP1_PRESCALE_R {
        CAP1_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - when set, CAP1 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap1_in_invert(&self) -> CAP1_IN_INVERT_R {
        CAP1_IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap1_en(&mut self) -> CAP1_EN_W {
        CAP1_EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap1_mode(&mut self) -> CAP1_MODE_W {
        CAP1_MODE_W::new(self)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap1_prescale(&mut self) -> CAP1_PRESCALE_W {
        CAP1_PRESCALE_W::new(self)
    }
    #[doc = "Bit 11 - when set, CAP1 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap1_in_invert(&mut self) -> CAP1_IN_INVERT_W {
        CAP1_IN_INVERT_W::new(self)
    }
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 1"]
    #[inline(always)]
    pub fn cap1_sw(&mut self) -> CAP1_SW_W {
        CAP1_SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture channel 1 configuration and enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch1_cfg](index.html) module"]
pub struct CAP_CH1_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch1_cfg::R](R) reader structure"]
impl crate::Readable for CAP_CH1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_ch1_cfg::W](W) writer structure"]
impl crate::Writable for CAP_CH1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_CH1_CFG to value 0"]
impl crate::Resettable for CAP_CH1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
