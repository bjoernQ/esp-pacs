#[doc = "Register `LSCH0_CONF0` reader"]
pub struct R(crate::R<LSCH0_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH0_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH0_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH0_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH0_CONF0` writer"]
pub struct W(crate::W<LSCH0_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH0_CONF0_SPEC>;
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
impl From<crate::W<LSCH0_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH0_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL_LSCH0` reader - reg_timer_sel_lsch0."]
pub type TIMER_SEL_LSCH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_SEL_LSCH0` writer - reg_timer_sel_lsch0."]
pub type TIMER_SEL_LSCH0_W<'a> = crate::FieldWriter<'a, u32, LSCH0_CONF0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SIG_OUT_EN_LSCH0` reader - reg_sig_out_en_lsch0."]
pub type SIG_OUT_EN_LSCH0_R = crate::BitReader<bool>;
#[doc = "Field `SIG_OUT_EN_LSCH0` writer - reg_sig_out_en_lsch0."]
pub type SIG_OUT_EN_LSCH0_W<'a> = crate::BitWriter<'a, u32, LSCH0_CONF0_SPEC, bool, 2>;
#[doc = "Field `IDLE_LV_LSCH0` reader - reg_idle_lv_lsch0."]
pub type IDLE_LV_LSCH0_R = crate::BitReader<bool>;
#[doc = "Field `IDLE_LV_LSCH0` writer - reg_idle_lv_lsch0."]
pub type IDLE_LV_LSCH0_W<'a> = crate::BitWriter<'a, u32, LSCH0_CONF0_SPEC, bool, 3>;
#[doc = "Field `PARA_UP_LSCH0` writer - reg_para_up_lsch0."]
pub type PARA_UP_LSCH0_W<'a> = crate::BitWriter<'a, u32, LSCH0_CONF0_SPEC, bool, 4>;
#[doc = "Field `OVF_NUM_LSCH0` reader - reg_ovf_num_lsch0."]
pub type OVF_NUM_LSCH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVF_NUM_LSCH0` writer - reg_ovf_num_lsch0."]
pub type OVF_NUM_LSCH0_W<'a> = crate::FieldWriter<'a, u32, LSCH0_CONF0_SPEC, u16, u16, 10, 5>;
#[doc = "Field `OVF_CNT_EN_LSCH0` reader - reg_ovf_cnt_en_lsch0."]
pub type OVF_CNT_EN_LSCH0_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_EN_LSCH0` writer - reg_ovf_cnt_en_lsch0."]
pub type OVF_CNT_EN_LSCH0_W<'a> = crate::BitWriter<'a, u32, LSCH0_CONF0_SPEC, bool, 15>;
#[doc = "Field `OVF_CNT_RESET_LSCH0` writer - reg_ovf_cnt_reset_lsch0."]
pub type OVF_CNT_RESET_LSCH0_W<'a> = crate::BitWriter<'a, u32, LSCH0_CONF0_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch0."]
    #[inline(always)]
    pub fn timer_sel_lsch0(&self) -> TIMER_SEL_LSCH0_R {
        TIMER_SEL_LSCH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch0."]
    #[inline(always)]
    pub fn sig_out_en_lsch0(&self) -> SIG_OUT_EN_LSCH0_R {
        SIG_OUT_EN_LSCH0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch0."]
    #[inline(always)]
    pub fn idle_lv_lsch0(&self) -> IDLE_LV_LSCH0_R {
        IDLE_LV_LSCH0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch0."]
    #[inline(always)]
    pub fn ovf_num_lsch0(&self) -> OVF_NUM_LSCH0_R {
        OVF_NUM_LSCH0_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch0(&self) -> OVF_CNT_EN_LSCH0_R {
        OVF_CNT_EN_LSCH0_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch0."]
    #[inline(always)]
    pub fn timer_sel_lsch0(&mut self) -> TIMER_SEL_LSCH0_W {
        TIMER_SEL_LSCH0_W::new(self)
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch0."]
    #[inline(always)]
    pub fn sig_out_en_lsch0(&mut self) -> SIG_OUT_EN_LSCH0_W {
        SIG_OUT_EN_LSCH0_W::new(self)
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch0."]
    #[inline(always)]
    pub fn idle_lv_lsch0(&mut self) -> IDLE_LV_LSCH0_W {
        IDLE_LV_LSCH0_W::new(self)
    }
    #[doc = "Bit 4 - reg_para_up_lsch0."]
    #[inline(always)]
    pub fn para_up_lsch0(&mut self) -> PARA_UP_LSCH0_W {
        PARA_UP_LSCH0_W::new(self)
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch0."]
    #[inline(always)]
    pub fn ovf_num_lsch0(&mut self) -> OVF_NUM_LSCH0_W {
        OVF_NUM_LSCH0_W::new(self)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch0(&mut self) -> OVF_CNT_EN_LSCH0_W {
        OVF_CNT_EN_LSCH0_W::new(self)
    }
    #[doc = "Bit 16 - reg_ovf_cnt_reset_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_reset_lsch0(&mut self) -> OVF_CNT_RESET_LSCH0_W {
        OVF_CNT_RESET_LSCH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH0_CONF0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_conf0](index.html) module"]
pub struct LSCH0_CONF0_SPEC;
impl crate::RegisterSpec for LSCH0_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch0_conf0::R](R) reader structure"]
impl crate::Readable for LSCH0_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch0_conf0::W](W) writer structure"]
impl crate::Writable for LSCH0_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH0_CONF0 to value 0"]
impl crate::Resettable for LSCH0_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
