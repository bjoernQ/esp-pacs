#[doc = "Register `TZ1_CFG0` reader"]
pub struct R(crate::R<TZ1_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZ1_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZ1_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZ1_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZ1_CFG0` writer"]
pub struct W(crate::W<TZ1_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZ1_CFG0_SPEC>;
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
impl From<crate::W<TZ1_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZ1_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZ1_SW_CBC` reader - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_SW_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_SW_CBC` writer - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_SW_CBC_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 0>;
#[doc = "Field `TZ1_F2_CBC` reader - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_F2_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_F2_CBC` writer - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_F2_CBC_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 1>;
#[doc = "Field `TZ1_F1_CBC` reader - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_F1_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_F1_CBC` writer - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_F1_CBC_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 2>;
#[doc = "Field `TZ1_F0_CBC` reader - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_F0_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_F0_CBC` writer - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ1_F0_CBC_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 3>;
#[doc = "Field `TZ1_SW_OST` reader - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_SW_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_SW_OST` writer - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_SW_OST_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 4>;
#[doc = "Field `TZ1_F2_OST` reader - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_F2_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_F2_OST` writer - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_F2_OST_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 5>;
#[doc = "Field `TZ1_F1_OST` reader - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_F1_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_F1_OST` writer - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_F1_OST_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 6>;
#[doc = "Field `TZ1_F0_OST` reader - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_F0_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_F0_OST` writer - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ1_F0_OST_W<'a> = crate::BitWriter<'a, u32, TZ1_CFG0_SPEC, bool, 7>;
#[doc = "Field `TZ1_A_CBC_D` reader - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_CBC_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_A_CBC_D` writer - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_CBC_D_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 8>;
#[doc = "Field `TZ1_A_CBC_U` reader - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_CBC_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_A_CBC_U` writer - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_CBC_U_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 10>;
#[doc = "Field `TZ1_A_OST_D` reader - One-shot mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_OST_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_A_OST_D` writer - One-shot mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_OST_D_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 12>;
#[doc = "Field `TZ1_A_OST_U` reader - One-shot mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_OST_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_A_OST_U` writer - One-shot mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_A_OST_U_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 14>;
#[doc = "Field `TZ1_B_CBC_D` reader - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_CBC_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_B_CBC_D` writer - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_CBC_D_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 16>;
#[doc = "Field `TZ1_B_CBC_U` reader - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_CBC_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_B_CBC_U` writer - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_CBC_U_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 18>;
#[doc = "Field `TZ1_B_OST_D` reader - One-shot mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_OST_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_B_OST_D` writer - One-shot mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_OST_D_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 20>;
#[doc = "Field `TZ1_B_OST_U` reader - One-shot mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_OST_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_B_OST_U` writer - One-shot mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub type TZ1_B_OST_U_W<'a> = crate::FieldWriter<'a, u32, TZ1_CFG0_SPEC, u8, u8, 2, 22>;
impl R {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_sw_cbc(&self) -> TZ1_SW_CBC_R {
        TZ1_SW_CBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f2_cbc(&self) -> TZ1_F2_CBC_R {
        TZ1_F2_CBC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f1_cbc(&self) -> TZ1_F1_CBC_R {
        TZ1_F1_CBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f0_cbc(&self) -> TZ1_F0_CBC_R {
        TZ1_F0_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_sw_ost(&self) -> TZ1_SW_OST_R {
        TZ1_SW_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f2_ost(&self) -> TZ1_F2_OST_R {
        TZ1_F2_OST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f1_ost(&self) -> TZ1_F1_OST_R {
        TZ1_F1_OST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f0_ost(&self) -> TZ1_F0_OST_R {
        TZ1_F0_OST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_cbc_d(&self) -> TZ1_A_CBC_D_R {
        TZ1_A_CBC_D_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_cbc_u(&self) -> TZ1_A_CBC_U_R {
        TZ1_A_CBC_U_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_ost_d(&self) -> TZ1_A_OST_D_R {
        TZ1_A_OST_D_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_ost_u(&self) -> TZ1_A_OST_U_R {
        TZ1_A_OST_U_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_cbc_d(&self) -> TZ1_B_CBC_D_R {
        TZ1_B_CBC_D_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_cbc_u(&self) -> TZ1_B_CBC_U_R {
        TZ1_B_CBC_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_ost_d(&self) -> TZ1_B_OST_D_R {
        TZ1_B_OST_D_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_ost_u(&self) -> TZ1_B_OST_U_R {
        TZ1_B_OST_U_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_sw_cbc(&mut self) -> TZ1_SW_CBC_W {
        TZ1_SW_CBC_W::new(self)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f2_cbc(&mut self) -> TZ1_F2_CBC_W {
        TZ1_F2_CBC_W::new(self)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f1_cbc(&mut self) -> TZ1_F1_CBC_W {
        TZ1_F1_CBC_W::new(self)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f0_cbc(&mut self) -> TZ1_F0_CBC_W {
        TZ1_F0_CBC_W::new(self)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_sw_ost(&mut self) -> TZ1_SW_OST_W {
        TZ1_SW_OST_W::new(self)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f2_ost(&mut self) -> TZ1_F2_OST_W {
        TZ1_F2_OST_W::new(self)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f1_ost(&mut self) -> TZ1_F1_OST_W {
        TZ1_F1_OST_W::new(self)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz1_f0_ost(&mut self) -> TZ1_F0_OST_W {
        TZ1_F0_OST_W::new(self)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_cbc_d(&mut self) -> TZ1_A_CBC_D_W {
        TZ1_A_CBC_D_W::new(self)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_cbc_u(&mut self) -> TZ1_A_CBC_U_W {
        TZ1_A_CBC_U_W::new(self)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM1A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_ost_d(&mut self) -> TZ1_A_OST_D_W {
        TZ1_A_OST_D_W::new(self)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM1A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_a_ost_u(&mut self) -> TZ1_A_OST_U_W {
        TZ1_A_OST_U_W::new(self)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_cbc_d(&mut self) -> TZ1_B_CBC_D_W {
        TZ1_B_CBC_D_W::new(self)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_cbc_u(&mut self) -> TZ1_B_CBC_U_W {
        TZ1_B_CBC_U_W::new(self)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM1B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_ost_d(&mut self) -> TZ1_B_OST_D_W {
        TZ1_B_OST_D_W::new(self)
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM1B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz1_b_ost_u(&mut self) -> TZ1_B_OST_U_W {
        TZ1_B_OST_U_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actions on PWM1A and PWM1B trip events\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tz1_cfg0](index.html) module"]
pub struct TZ1_CFG0_SPEC;
impl crate::RegisterSpec for TZ1_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tz1_cfg0::R](R) reader structure"]
impl crate::Readable for TZ1_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tz1_cfg0::W](W) writer structure"]
impl crate::Writable for TZ1_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZ1_CFG0 to value 0"]
impl crate::Resettable for TZ1_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
