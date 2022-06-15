#[doc = "Register `LSCH5_CONF0` reader"]
pub struct R(crate::R<LSCH5_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH5_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH5_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH5_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH5_CONF0` writer"]
pub struct W(crate::W<LSCH5_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH5_CONF0_SPEC>;
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
impl From<crate::W<LSCH5_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH5_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL_LSCH5` reader - There are four low speed timers the two bits are used to select one of them for low speed channel5. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
pub type TIMER_SEL_LSCH5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_SEL_LSCH5` writer - There are four low speed timers the two bits are used to select one of them for low speed channel5. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
pub type TIMER_SEL_LSCH5_W<'a> = crate::FieldWriter<'a, u32, LSCH5_CONF0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SIG_OUT_EN_LSCH5` reader - This is the output enable control bit for low speed channel5."]
pub type SIG_OUT_EN_LSCH5_R = crate::BitReader<bool>;
#[doc = "Field `SIG_OUT_EN_LSCH5` writer - This is the output enable control bit for low speed channel5."]
pub type SIG_OUT_EN_LSCH5_W<'a> = crate::BitWriter<'a, u32, LSCH5_CONF0_SPEC, bool, 2>;
#[doc = "Field `IDLE_LV_LSCH5` reader - This bit is used to control the output value when low speed channel5 is off."]
pub type IDLE_LV_LSCH5_R = crate::BitReader<bool>;
#[doc = "Field `IDLE_LV_LSCH5` writer - This bit is used to control the output value when low speed channel5 is off."]
pub type IDLE_LV_LSCH5_W<'a> = crate::BitWriter<'a, u32, LSCH5_CONF0_SPEC, bool, 3>;
#[doc = "Field `PARA_UP_LSCH5` reader - This bit is used to update register LEDC_LSCH5_HPOINT and LEDC_LSCH5_DUTY for low speed channel5."]
pub type PARA_UP_LSCH5_R = crate::BitReader<bool>;
#[doc = "Field `PARA_UP_LSCH5` writer - This bit is used to update register LEDC_LSCH5_HPOINT and LEDC_LSCH5_DUTY for low speed channel5."]
pub type PARA_UP_LSCH5_W<'a> = crate::BitWriter<'a, u32, LSCH5_CONF0_SPEC, bool, 4>;
impl R {
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel5. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    pub fn timer_sel_lsch5(&self) -> TIMER_SEL_LSCH5_R {
        TIMER_SEL_LSCH5_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel5."]
    #[inline(always)]
    pub fn sig_out_en_lsch5(&self) -> SIG_OUT_EN_LSCH5_R {
        SIG_OUT_EN_LSCH5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel5 is off."]
    #[inline(always)]
    pub fn idle_lv_lsch5(&self) -> IDLE_LV_LSCH5_R {
        IDLE_LV_LSCH5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH5_HPOINT and LEDC_LSCH5_DUTY for low speed channel5."]
    #[inline(always)]
    pub fn para_up_lsch5(&self) -> PARA_UP_LSCH5_R {
        PARA_UP_LSCH5_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel5. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    pub fn timer_sel_lsch5(&mut self) -> TIMER_SEL_LSCH5_W {
        TIMER_SEL_LSCH5_W::new(self)
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel5."]
    #[inline(always)]
    pub fn sig_out_en_lsch5(&mut self) -> SIG_OUT_EN_LSCH5_W {
        SIG_OUT_EN_LSCH5_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel5 is off."]
    #[inline(always)]
    pub fn idle_lv_lsch5(&mut self) -> IDLE_LV_LSCH5_W {
        IDLE_LV_LSCH5_W::new(self)
    }
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH5_HPOINT and LEDC_LSCH5_DUTY for low speed channel5."]
    #[inline(always)]
    pub fn para_up_lsch5(&mut self) -> PARA_UP_LSCH5_W {
        PARA_UP_LSCH5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_conf0](index.html) module"]
pub struct LSCH5_CONF0_SPEC;
impl crate::RegisterSpec for LSCH5_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch5_conf0::R](R) reader structure"]
impl crate::Readable for LSCH5_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch5_conf0::W](W) writer structure"]
impl crate::Writable for LSCH5_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH5_CONF0 to value 0"]
impl crate::Resettable for LSCH5_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
