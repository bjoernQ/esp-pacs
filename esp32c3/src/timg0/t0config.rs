#[doc = "Register `T0CONFIG` reader"]
pub struct R(crate::R<T0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0CONFIG` writer"]
pub struct W(crate::W<T0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0CONFIG_SPEC>;
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
impl From<crate::W<T0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_USE_XTAL` reader - reg_t0_use_xtal."]
pub type T0_USE_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `T0_USE_XTAL` writer - reg_t0_use_xtal."]
pub type T0_USE_XTAL_W<'a> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, 9>;
#[doc = "Field `T0_ALARM_EN` reader - reg_t0_alarm_en."]
pub type T0_ALARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `T0_ALARM_EN` writer - reg_t0_alarm_en."]
pub type T0_ALARM_EN_W<'a> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, 10>;
#[doc = "Field `T0_DIVCNT_RST` writer - reg_t0_divcnt_rst."]
pub type T0_DIVCNT_RST_W<'a> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, 12>;
#[doc = "Field `T0_DIVIDER` reader - reg_t0_divider."]
pub type T0_DIVIDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T0_DIVIDER` writer - reg_t0_divider."]
pub type T0_DIVIDER_W<'a> = crate::FieldWriter<'a, u32, T0CONFIG_SPEC, u16, u16, 16, 13>;
#[doc = "Field `T0_AUTORELOAD` reader - reg_t0_autoreload."]
pub type T0_AUTORELOAD_R = crate::BitReader<bool>;
#[doc = "Field `T0_AUTORELOAD` writer - reg_t0_autoreload."]
pub type T0_AUTORELOAD_W<'a> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, 29>;
#[doc = "Field `T0_INCREASE` reader - reg_t0_increase."]
pub type T0_INCREASE_R = crate::BitReader<bool>;
#[doc = "Field `T0_INCREASE` writer - reg_t0_increase."]
pub type T0_INCREASE_W<'a> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, 30>;
#[doc = "Field `T0_EN` reader - reg_t0_en."]
pub type T0_EN_R = crate::BitReader<bool>;
#[doc = "Field `T0_EN` writer - reg_t0_en."]
pub type T0_EN_W<'a> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 9 - reg_t0_use_xtal."]
    #[inline(always)]
    pub fn t0_use_xtal(&self) -> T0_USE_XTAL_R {
        T0_USE_XTAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_t0_alarm_en."]
    #[inline(always)]
    pub fn t0_alarm_en(&self) -> T0_ALARM_EN_R {
        T0_ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - reg_t0_divider."]
    #[inline(always)]
    pub fn t0_divider(&self) -> T0_DIVIDER_R {
        T0_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - reg_t0_autoreload."]
    #[inline(always)]
    pub fn t0_autoreload(&self) -> T0_AUTORELOAD_R {
        T0_AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reg_t0_increase."]
    #[inline(always)]
    pub fn t0_increase(&self) -> T0_INCREASE_R {
        T0_INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_t0_en."]
    #[inline(always)]
    pub fn t0_en(&self) -> T0_EN_R {
        T0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - reg_t0_use_xtal."]
    #[inline(always)]
    pub fn t0_use_xtal(&mut self) -> T0_USE_XTAL_W {
        T0_USE_XTAL_W::new(self)
    }
    #[doc = "Bit 10 - reg_t0_alarm_en."]
    #[inline(always)]
    pub fn t0_alarm_en(&mut self) -> T0_ALARM_EN_W {
        T0_ALARM_EN_W::new(self)
    }
    #[doc = "Bit 12 - reg_t0_divcnt_rst."]
    #[inline(always)]
    pub fn t0_divcnt_rst(&mut self) -> T0_DIVCNT_RST_W {
        T0_DIVCNT_RST_W::new(self)
    }
    #[doc = "Bits 13:28 - reg_t0_divider."]
    #[inline(always)]
    pub fn t0_divider(&mut self) -> T0_DIVIDER_W {
        T0_DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - reg_t0_autoreload."]
    #[inline(always)]
    pub fn t0_autoreload(&mut self) -> T0_AUTORELOAD_W {
        T0_AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - reg_t0_increase."]
    #[inline(always)]
    pub fn t0_increase(&mut self) -> T0_INCREASE_W {
        T0_INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - reg_t0_en."]
    #[inline(always)]
    pub fn t0_en(&mut self) -> T0_EN_W {
        T0_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_T0CONFIG_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0config](index.html) module"]
pub struct T0CONFIG_SPEC;
impl crate::RegisterSpec for T0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0config::R](R) reader structure"]
impl crate::Readable for T0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0config::W](W) writer structure"]
impl crate::Writable for T0CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0CONFIG to value 0x6000_2000"]
impl crate::Resettable for T0CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_2000
    }
}
