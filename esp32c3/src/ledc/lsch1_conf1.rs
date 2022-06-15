#[doc = "Register `LSCH1_CONF1` reader"]
pub struct R(crate::R<LSCH1_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH1_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH1_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH1_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH1_CONF1` writer"]
pub struct W(crate::W<LSCH1_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH1_CONF1_SPEC>;
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
impl From<crate::W<LSCH1_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH1_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_SCALE_LSCH1` reader - reg_duty_scale_lsch1."]
pub type DUTY_SCALE_LSCH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_SCALE_LSCH1` writer - reg_duty_scale_lsch1."]
pub type DUTY_SCALE_LSCH1_W<'a> = crate::FieldWriter<'a, u32, LSCH1_CONF1_SPEC, u16, u16, 10, 0>;
#[doc = "Field `DUTY_CYCLE_LSCH1` reader - reg_duty_cycle_lsch1."]
pub type DUTY_CYCLE_LSCH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_CYCLE_LSCH1` writer - reg_duty_cycle_lsch1."]
pub type DUTY_CYCLE_LSCH1_W<'a> = crate::FieldWriter<'a, u32, LSCH1_CONF1_SPEC, u16, u16, 10, 10>;
#[doc = "Field `DUTY_NUM_LSCH1` reader - reg_duty_num_lsch1."]
pub type DUTY_NUM_LSCH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_NUM_LSCH1` writer - reg_duty_num_lsch1."]
pub type DUTY_NUM_LSCH1_W<'a> = crate::FieldWriter<'a, u32, LSCH1_CONF1_SPEC, u16, u16, 10, 20>;
#[doc = "Field `DUTY_INC_LSCH1` reader - reg_duty_inc_lsch1."]
pub type DUTY_INC_LSCH1_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_INC_LSCH1` writer - reg_duty_inc_lsch1."]
pub type DUTY_INC_LSCH1_W<'a> = crate::BitWriter<'a, u32, LSCH1_CONF1_SPEC, bool, 30>;
#[doc = "Field `DUTY_START_LSCH1` reader - reg_duty_start_lsch1."]
pub type DUTY_START_LSCH1_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_START_LSCH1` writer - reg_duty_start_lsch1."]
pub type DUTY_START_LSCH1_W<'a> = crate::BitWriter<'a, u32, LSCH1_CONF1_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - reg_duty_scale_lsch1."]
    #[inline(always)]
    pub fn duty_scale_lsch1(&self) -> DUTY_SCALE_LSCH1_R {
        DUTY_SCALE_LSCH1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - reg_duty_cycle_lsch1."]
    #[inline(always)]
    pub fn duty_cycle_lsch1(&self) -> DUTY_CYCLE_LSCH1_R {
        DUTY_CYCLE_LSCH1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - reg_duty_num_lsch1."]
    #[inline(always)]
    pub fn duty_num_lsch1(&self) -> DUTY_NUM_LSCH1_R {
        DUTY_NUM_LSCH1_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - reg_duty_inc_lsch1."]
    #[inline(always)]
    pub fn duty_inc_lsch1(&self) -> DUTY_INC_LSCH1_R {
        DUTY_INC_LSCH1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_duty_start_lsch1."]
    #[inline(always)]
    pub fn duty_start_lsch1(&self) -> DUTY_START_LSCH1_R {
        DUTY_START_LSCH1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - reg_duty_scale_lsch1."]
    #[inline(always)]
    pub fn duty_scale_lsch1(&mut self) -> DUTY_SCALE_LSCH1_W {
        DUTY_SCALE_LSCH1_W::new(self)
    }
    #[doc = "Bits 10:19 - reg_duty_cycle_lsch1."]
    #[inline(always)]
    pub fn duty_cycle_lsch1(&mut self) -> DUTY_CYCLE_LSCH1_W {
        DUTY_CYCLE_LSCH1_W::new(self)
    }
    #[doc = "Bits 20:29 - reg_duty_num_lsch1."]
    #[inline(always)]
    pub fn duty_num_lsch1(&mut self) -> DUTY_NUM_LSCH1_W {
        DUTY_NUM_LSCH1_W::new(self)
    }
    #[doc = "Bit 30 - reg_duty_inc_lsch1."]
    #[inline(always)]
    pub fn duty_inc_lsch1(&mut self) -> DUTY_INC_LSCH1_W {
        DUTY_INC_LSCH1_W::new(self)
    }
    #[doc = "Bit 31 - reg_duty_start_lsch1."]
    #[inline(always)]
    pub fn duty_start_lsch1(&mut self) -> DUTY_START_LSCH1_W {
        DUTY_START_LSCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH1_CONF1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_conf1](index.html) module"]
pub struct LSCH1_CONF1_SPEC;
impl crate::RegisterSpec for LSCH1_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch1_conf1::R](R) reader structure"]
impl crate::Readable for LSCH1_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch1_conf1::W](W) writer structure"]
impl crate::Writable for LSCH1_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH1_CONF1 to value 0x4000_0000"]
impl crate::Resettable for LSCH1_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
