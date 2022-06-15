#[doc = "Register `U1_CONF0` reader"]
pub struct R(crate::R<U1_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U1_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U1_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1_CONF0` writer"]
pub struct W(crate::W<U1_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1_CONF0_SPEC>;
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
impl From<crate::W<U1_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U1_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_THRES_U1` reader - This register is used to filter pluse whose width is smaller than this value for unit1."]
pub type FILTER_THRES_U1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FILTER_THRES_U1` writer - This register is used to filter pluse whose width is smaller than this value for unit1."]
pub type FILTER_THRES_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u16, u16, 10, 0>;
#[doc = "Field `FILTER_EN_U1` reader - This is the enable bit for filtering input signals for unit1."]
pub type FILTER_EN_U1_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_EN_U1` writer - This is the enable bit for filtering input signals for unit1."]
pub type FILTER_EN_U1_W<'a> = crate::BitWriter<'a, u32, U1_CONF0_SPEC, bool, 10>;
#[doc = "Field `THR_ZERO_EN_U1` reader - This is the enable bit for comparing unit1's count with 0 value."]
pub type THR_ZERO_EN_U1_R = crate::BitReader<bool>;
#[doc = "Field `THR_ZERO_EN_U1` writer - This is the enable bit for comparing unit1's count with 0 value."]
pub type THR_ZERO_EN_U1_W<'a> = crate::BitWriter<'a, u32, U1_CONF0_SPEC, bool, 11>;
#[doc = "Field `THR_H_LIM_EN_U1` reader - This is the enable bit for comparing unit1's count with thr_h_lim value."]
pub type THR_H_LIM_EN_U1_R = crate::BitReader<bool>;
#[doc = "Field `THR_H_LIM_EN_U1` writer - This is the enable bit for comparing unit1's count with thr_h_lim value."]
pub type THR_H_LIM_EN_U1_W<'a> = crate::BitWriter<'a, u32, U1_CONF0_SPEC, bool, 12>;
#[doc = "Field `THR_L_LIM_EN_U1` reader - This is the enable bit for comparing unit1's count with thr_l_lim value."]
pub type THR_L_LIM_EN_U1_R = crate::BitReader<bool>;
#[doc = "Field `THR_L_LIM_EN_U1` writer - This is the enable bit for comparing unit1's count with thr_l_lim value."]
pub type THR_L_LIM_EN_U1_W<'a> = crate::BitWriter<'a, u32, U1_CONF0_SPEC, bool, 13>;
#[doc = "Field `THR_THRES0_EN_U1` reader - This is the enable bit for comparing unit1's count with thres0 value."]
pub type THR_THRES0_EN_U1_R = crate::BitReader<bool>;
#[doc = "Field `THR_THRES0_EN_U1` writer - This is the enable bit for comparing unit1's count with thres0 value."]
pub type THR_THRES0_EN_U1_W<'a> = crate::BitWriter<'a, u32, U1_CONF0_SPEC, bool, 14>;
#[doc = "Field `THR_THRES1_EN_U1` reader - This is the enable bit for comparing unit1's count with thres1 value ."]
pub type THR_THRES1_EN_U1_R = crate::BitReader<bool>;
#[doc = "Field `THR_THRES1_EN_U1` writer - This is the enable bit for comparing unit1's count with thres1 value ."]
pub type THR_THRES1_EN_U1_W<'a> = crate::BitWriter<'a, u32, U1_CONF0_SPEC, bool, 15>;
#[doc = "Field `CH0_NEG_MODE_U1` reader - This register is used to control the mode of channel0's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH0_NEG_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_NEG_MODE_U1` writer - This register is used to control the mode of channel0's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH0_NEG_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 16>;
#[doc = "Field `CH0_POS_MODE_U1` reader - This register is used to control the mode of channel0's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH0_POS_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_POS_MODE_U1` writer - This register is used to control the mode of channel0's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH0_POS_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 18>;
#[doc = "Field `CH0_HCTRL_MODE_U1` reader - This register is used to control the mode of channel0's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_HCTRL_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_HCTRL_MODE_U1` writer - This register is used to control the mode of channel0's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_HCTRL_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 20>;
#[doc = "Field `CH0_LCTRL_MODE_U1` reader - This register is used to control the mode of channel0's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_LCTRL_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_LCTRL_MODE_U1` writer - This register is used to control the mode of channel0's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_LCTRL_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 22>;
#[doc = "Field `CH1_NEG_MODE_U1` reader - This register is used to control the mode of channel1's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH1_NEG_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_NEG_MODE_U1` writer - This register is used to control the mode of channel1's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH1_NEG_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 24>;
#[doc = "Field `CH1_POS_MODE_U1` reader - This register is used to control the mode of channel1's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH1_POS_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_POS_MODE_U1` writer - This register is used to control the mode of channel1's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH1_POS_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 26>;
#[doc = "Field `CH1_HCTRL_MODE_U1` reader - This register is used to control the mode of channel1's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_HCTRL_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_HCTRL_MODE_U1` writer - This register is used to control the mode of channel1's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_HCTRL_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 28>;
#[doc = "Field `CH1_LCTRL_MODE_U1` reader - This register is used to control the mode of channel1's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_LCTRL_MODE_U1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_LCTRL_MODE_U1` writer - This register is used to control the mode of channel1's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_LCTRL_MODE_U1_W<'a> = crate::FieldWriter<'a, u32, U1_CONF0_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit1."]
    #[inline(always)]
    pub fn filter_thres_u1(&self) -> FILTER_THRES_U1_R {
        FILTER_THRES_U1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit1."]
    #[inline(always)]
    pub fn filter_en_u1(&self) -> FILTER_EN_U1_R {
        FILTER_EN_U1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit1's count with 0 value."]
    #[inline(always)]
    pub fn thr_zero_en_u1(&self) -> THR_ZERO_EN_U1_R {
        THR_ZERO_EN_U1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit1's count with thr_h_lim value."]
    #[inline(always)]
    pub fn thr_h_lim_en_u1(&self) -> THR_H_LIM_EN_U1_R {
        THR_H_LIM_EN_U1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit1's count with thr_l_lim value."]
    #[inline(always)]
    pub fn thr_l_lim_en_u1(&self) -> THR_L_LIM_EN_U1_R {
        THR_L_LIM_EN_U1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit1's count with thres0 value."]
    #[inline(always)]
    pub fn thr_thres0_en_u1(&self) -> THR_THRES0_EN_U1_R {
        THR_THRES0_EN_U1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit1's count with thres1 value ."]
    #[inline(always)]
    pub fn thr_thres1_en_u1(&self) -> THR_THRES1_EN_U1_R {
        THR_THRES1_EN_U1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_neg_mode_u1(&self) -> CH0_NEG_MODE_U1_R {
        CH0_NEG_MODE_U1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_pos_mode_u1(&self) -> CH0_POS_MODE_U1_R {
        CH0_POS_MODE_U1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u1(&self) -> CH0_HCTRL_MODE_U1_R {
        CH0_HCTRL_MODE_U1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u1(&self) -> CH0_LCTRL_MODE_U1_R {
        CH0_LCTRL_MODE_U1_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_neg_mode_u1(&self) -> CH1_NEG_MODE_U1_R {
        CH1_NEG_MODE_U1_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_pos_mode_u1(&self) -> CH1_POS_MODE_U1_R {
        CH1_POS_MODE_U1_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u1(&self) -> CH1_HCTRL_MODE_U1_R {
        CH1_HCTRL_MODE_U1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u1(&self) -> CH1_LCTRL_MODE_U1_R {
        CH1_LCTRL_MODE_U1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit1."]
    #[inline(always)]
    pub fn filter_thres_u1(&mut self) -> FILTER_THRES_U1_W {
        FILTER_THRES_U1_W::new(self)
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit1."]
    #[inline(always)]
    pub fn filter_en_u1(&mut self) -> FILTER_EN_U1_W {
        FILTER_EN_U1_W::new(self)
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit1's count with 0 value."]
    #[inline(always)]
    pub fn thr_zero_en_u1(&mut self) -> THR_ZERO_EN_U1_W {
        THR_ZERO_EN_U1_W::new(self)
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit1's count with thr_h_lim value."]
    #[inline(always)]
    pub fn thr_h_lim_en_u1(&mut self) -> THR_H_LIM_EN_U1_W {
        THR_H_LIM_EN_U1_W::new(self)
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit1's count with thr_l_lim value."]
    #[inline(always)]
    pub fn thr_l_lim_en_u1(&mut self) -> THR_L_LIM_EN_U1_W {
        THR_L_LIM_EN_U1_W::new(self)
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit1's count with thres0 value."]
    #[inline(always)]
    pub fn thr_thres0_en_u1(&mut self) -> THR_THRES0_EN_U1_W {
        THR_THRES0_EN_U1_W::new(self)
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit1's count with thres1 value ."]
    #[inline(always)]
    pub fn thr_thres1_en_u1(&mut self) -> THR_THRES1_EN_U1_W {
        THR_THRES1_EN_U1_W::new(self)
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_neg_mode_u1(&mut self) -> CH0_NEG_MODE_U1_W {
        CH0_NEG_MODE_U1_W::new(self)
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_pos_mode_u1(&mut self) -> CH0_POS_MODE_U1_W {
        CH0_POS_MODE_U1_W::new(self)
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u1(&mut self) -> CH0_HCTRL_MODE_U1_W {
        CH0_HCTRL_MODE_U1_W::new(self)
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u1(&mut self) -> CH0_LCTRL_MODE_U1_W {
        CH0_LCTRL_MODE_U1_W::new(self)
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit1. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_neg_mode_u1(&mut self) -> CH1_NEG_MODE_U1_W {
        CH1_NEG_MODE_U1_W::new(self)
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit1. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_pos_mode_u1(&mut self) -> CH1_POS_MODE_U1_W {
        CH1_POS_MODE_U1_W::new(self)
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u1(&mut self) -> CH1_HCTRL_MODE_U1_W {
        CH1_HCTRL_MODE_U1_W::new(self)
    }
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit1. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u1(&mut self) -> CH1_LCTRL_MODE_U1_W {
        CH1_LCTRL_MODE_U1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1_conf0](index.html) module"]
pub struct U1_CONF0_SPEC;
impl crate::RegisterSpec for U1_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1_conf0::R](R) reader structure"]
impl crate::Readable for U1_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1_conf0::W](W) writer structure"]
impl crate::Writable for U1_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1_CONF0 to value 0x3c10"]
impl crate::Resettable for U1_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c10
    }
}
