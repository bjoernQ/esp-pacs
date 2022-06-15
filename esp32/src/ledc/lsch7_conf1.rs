#[doc = "Register `LSCH7_CONF1` reader"]
pub struct R(crate::R<LSCH7_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH7_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH7_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH7_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH7_CONF1` writer"]
pub struct W(crate::W<LSCH7_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH7_CONF1_SPEC>;
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
impl From<crate::W<LSCH7_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH7_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_SCALE_LSCH7` reader - This register controls the increase or decrease step scale for low speed channel7."]
pub type DUTY_SCALE_LSCH7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_SCALE_LSCH7` writer - This register controls the increase or decrease step scale for low speed channel7."]
pub type DUTY_SCALE_LSCH7_W<'a> = crate::FieldWriter<'a, u32, LSCH7_CONF1_SPEC, u16, u16, 10, 0>;
#[doc = "Field `DUTY_CYCLE_LSCH7` reader - This register is used to increase or decrease the duty every reg_duty_cycle_lsch7 cycles for low speed channel7."]
pub type DUTY_CYCLE_LSCH7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_CYCLE_LSCH7` writer - This register is used to increase or decrease the duty every reg_duty_cycle_lsch7 cycles for low speed channel7."]
pub type DUTY_CYCLE_LSCH7_W<'a> = crate::FieldWriter<'a, u32, LSCH7_CONF1_SPEC, u16, u16, 10, 10>;
#[doc = "Field `DUTY_NUM_LSCH7` reader - This register is used to control the num of increased or decreased times for low speed channel4."]
pub type DUTY_NUM_LSCH7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_NUM_LSCH7` writer - This register is used to control the num of increased or decreased times for low speed channel4."]
pub type DUTY_NUM_LSCH7_W<'a> = crate::FieldWriter<'a, u32, LSCH7_CONF1_SPEC, u16, u16, 10, 20>;
#[doc = "Field `DUTY_INC_LSCH7` reader - This register is used to increase the duty of output signal or decrease the duty of output signal for low speed channel4."]
pub type DUTY_INC_LSCH7_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_INC_LSCH7` writer - This register is used to increase the duty of output signal or decrease the duty of output signal for low speed channel4."]
pub type DUTY_INC_LSCH7_W<'a> = crate::BitWriter<'a, u32, LSCH7_CONF1_SPEC, bool, 30>;
#[doc = "Field `DUTY_START_LSCH7` reader - When reg_duty_num_hsch4 reg_duty_cycle_hsch4 and reg_duty_scale_hsch4 has been configured. these register won't take effect until set reg_duty_start_hsch4. this bit is automatically cleared by hardware."]
pub type DUTY_START_LSCH7_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_START_LSCH7` writer - When reg_duty_num_hsch4 reg_duty_cycle_hsch4 and reg_duty_scale_hsch4 has been configured. these register won't take effect until set reg_duty_start_hsch4. this bit is automatically cleared by hardware."]
pub type DUTY_START_LSCH7_W<'a> = crate::BitWriter<'a, u32, LSCH7_CONF1_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for low speed channel7."]
    #[inline(always)]
    pub fn duty_scale_lsch7(&self) -> DUTY_SCALE_LSCH7_R {
        DUTY_SCALE_LSCH7_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_lsch7 cycles for low speed channel7."]
    #[inline(always)]
    pub fn duty_cycle_lsch7(&self) -> DUTY_CYCLE_LSCH7_R {
        DUTY_CYCLE_LSCH7_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for low speed channel4."]
    #[inline(always)]
    pub fn duty_num_lsch7(&self) -> DUTY_NUM_LSCH7_R {
        DUTY_NUM_LSCH7_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for low speed channel4."]
    #[inline(always)]
    pub fn duty_inc_lsch7(&self) -> DUTY_INC_LSCH7_R {
        DUTY_INC_LSCH7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch4 reg_duty_cycle_hsch4 and reg_duty_scale_hsch4 has been configured. these register won't take effect until set reg_duty_start_hsch4. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_lsch7(&self) -> DUTY_START_LSCH7_R {
        DUTY_START_LSCH7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for low speed channel7."]
    #[inline(always)]
    pub fn duty_scale_lsch7(&mut self) -> DUTY_SCALE_LSCH7_W {
        DUTY_SCALE_LSCH7_W::new(self)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_lsch7 cycles for low speed channel7."]
    #[inline(always)]
    pub fn duty_cycle_lsch7(&mut self) -> DUTY_CYCLE_LSCH7_W {
        DUTY_CYCLE_LSCH7_W::new(self)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for low speed channel4."]
    #[inline(always)]
    pub fn duty_num_lsch7(&mut self) -> DUTY_NUM_LSCH7_W {
        DUTY_NUM_LSCH7_W::new(self)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for low speed channel4."]
    #[inline(always)]
    pub fn duty_inc_lsch7(&mut self) -> DUTY_INC_LSCH7_W {
        DUTY_INC_LSCH7_W::new(self)
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch4 reg_duty_cycle_hsch4 and reg_duty_scale_hsch4 has been configured. these register won't take effect until set reg_duty_start_hsch4. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_lsch7(&mut self) -> DUTY_START_LSCH7_W {
        DUTY_START_LSCH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch7_conf1](index.html) module"]
pub struct LSCH7_CONF1_SPEC;
impl crate::RegisterSpec for LSCH7_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch7_conf1::R](R) reader structure"]
impl crate::Readable for LSCH7_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch7_conf1::W](W) writer structure"]
impl crate::Writable for LSCH7_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH7_CONF1 to value 0x4000_0000"]
impl crate::Resettable for LSCH7_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
