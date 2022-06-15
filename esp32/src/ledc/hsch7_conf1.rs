#[doc = "Register `HSCH7_CONF1` reader"]
pub struct R(crate::R<HSCH7_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH7_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH7_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH7_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH7_CONF1` writer"]
pub struct W(crate::W<HSCH7_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH7_CONF1_SPEC>;
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
impl From<crate::W<HSCH7_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH7_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_SCALE_HSCH7` reader - This register controls the increase or decrease step scale for high speed channel7."]
pub type DUTY_SCALE_HSCH7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_SCALE_HSCH7` writer - This register controls the increase or decrease step scale for high speed channel7."]
pub type DUTY_SCALE_HSCH7_W<'a> = crate::FieldWriter<'a, u32, HSCH7_CONF1_SPEC, u16, u16, 10, 0>;
#[doc = "Field `DUTY_CYCLE_HSCH7` reader - This register is used to increase or decrease the duty every reg_duty_cycle_hsch7 cycles for high speed channel7."]
pub type DUTY_CYCLE_HSCH7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_CYCLE_HSCH7` writer - This register is used to increase or decrease the duty every reg_duty_cycle_hsch7 cycles for high speed channel7."]
pub type DUTY_CYCLE_HSCH7_W<'a> = crate::FieldWriter<'a, u32, HSCH7_CONF1_SPEC, u16, u16, 10, 10>;
#[doc = "Field `DUTY_NUM_HSCH7` reader - This register is used to control the num of increased or decreased times for high speed channel6."]
pub type DUTY_NUM_HSCH7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_NUM_HSCH7` writer - This register is used to control the num of increased or decreased times for high speed channel6."]
pub type DUTY_NUM_HSCH7_W<'a> = crate::FieldWriter<'a, u32, HSCH7_CONF1_SPEC, u16, u16, 10, 20>;
#[doc = "Field `DUTY_INC_HSCH7` reader - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel6."]
pub type DUTY_INC_HSCH7_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_INC_HSCH7` writer - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel6."]
pub type DUTY_INC_HSCH7_W<'a> = crate::BitWriter<'a, u32, HSCH7_CONF1_SPEC, bool, 30>;
#[doc = "Field `DUTY_START_HSCH7` reader - When reg_duty_num_hsch1 reg_duty_cycle_hsch1 and reg_duty_scale_hsch1 has been configured. these register won't take effect until set reg_duty_start_hsch1. this bit is automatically cleared by hardware."]
pub type DUTY_START_HSCH7_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_START_HSCH7` writer - When reg_duty_num_hsch1 reg_duty_cycle_hsch1 and reg_duty_scale_hsch1 has been configured. these register won't take effect until set reg_duty_start_hsch1. this bit is automatically cleared by hardware."]
pub type DUTY_START_HSCH7_W<'a> = crate::BitWriter<'a, u32, HSCH7_CONF1_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel7."]
    #[inline(always)]
    pub fn duty_scale_hsch7(&self) -> DUTY_SCALE_HSCH7_R {
        DUTY_SCALE_HSCH7_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch7 cycles for high speed channel7."]
    #[inline(always)]
    pub fn duty_cycle_hsch7(&self) -> DUTY_CYCLE_HSCH7_R {
        DUTY_CYCLE_HSCH7_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel6."]
    #[inline(always)]
    pub fn duty_num_hsch7(&self) -> DUTY_NUM_HSCH7_R {
        DUTY_NUM_HSCH7_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel6."]
    #[inline(always)]
    pub fn duty_inc_hsch7(&self) -> DUTY_INC_HSCH7_R {
        DUTY_INC_HSCH7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch1 reg_duty_cycle_hsch1 and reg_duty_scale_hsch1 has been configured. these register won't take effect until set reg_duty_start_hsch1. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_hsch7(&self) -> DUTY_START_HSCH7_R {
        DUTY_START_HSCH7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel7."]
    #[inline(always)]
    pub fn duty_scale_hsch7(&mut self) -> DUTY_SCALE_HSCH7_W {
        DUTY_SCALE_HSCH7_W::new(self)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch7 cycles for high speed channel7."]
    #[inline(always)]
    pub fn duty_cycle_hsch7(&mut self) -> DUTY_CYCLE_HSCH7_W {
        DUTY_CYCLE_HSCH7_W::new(self)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel6."]
    #[inline(always)]
    pub fn duty_num_hsch7(&mut self) -> DUTY_NUM_HSCH7_W {
        DUTY_NUM_HSCH7_W::new(self)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel6."]
    #[inline(always)]
    pub fn duty_inc_hsch7(&mut self) -> DUTY_INC_HSCH7_W {
        DUTY_INC_HSCH7_W::new(self)
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch1 reg_duty_cycle_hsch1 and reg_duty_scale_hsch1 has been configured. these register won't take effect until set reg_duty_start_hsch1. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_hsch7(&mut self) -> DUTY_START_HSCH7_W {
        DUTY_START_HSCH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch7_conf1](index.html) module"]
pub struct HSCH7_CONF1_SPEC;
impl crate::RegisterSpec for HSCH7_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch7_conf1::R](R) reader structure"]
impl crate::Readable for HSCH7_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch7_conf1::W](W) writer structure"]
impl crate::Writable for HSCH7_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH7_CONF1 to value 0x4000_0000"]
impl crate::Resettable for HSCH7_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
