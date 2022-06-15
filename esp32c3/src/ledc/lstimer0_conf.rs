#[doc = "Register `LSTIMER0_CONF` reader"]
pub struct R(crate::R<LSTIMER0_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER0_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER0_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER0_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTIMER0_CONF` writer"]
pub struct W(crate::W<LSTIMER0_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTIMER0_CONF_SPEC>;
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
impl From<crate::W<LSTIMER0_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTIMER0_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTIMER0_DUTY_RES` reader - reg_lstimer0_duty_res."]
pub type LSTIMER0_DUTY_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSTIMER0_DUTY_RES` writer - reg_lstimer0_duty_res."]
pub type LSTIMER0_DUTY_RES_W<'a> = crate::FieldWriter<'a, u32, LSTIMER0_CONF_SPEC, u8, u8, 4, 0>;
#[doc = "Field `CLK_DIV_LSTIMER0` reader - reg_clk_div_lstimer0."]
pub type CLK_DIV_LSTIMER0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CLK_DIV_LSTIMER0` writer - reg_clk_div_lstimer0."]
pub type CLK_DIV_LSTIMER0_W<'a> = crate::FieldWriter<'a, u32, LSTIMER0_CONF_SPEC, u32, u32, 18, 4>;
#[doc = "Field `LSTIMER0_PAUSE` reader - reg_lstimer0_pause."]
pub type LSTIMER0_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `LSTIMER0_PAUSE` writer - reg_lstimer0_pause."]
pub type LSTIMER0_PAUSE_W<'a> = crate::BitWriter<'a, u32, LSTIMER0_CONF_SPEC, bool, 22>;
#[doc = "Field `LSTIMER0_RST` reader - reg_lstimer0_rst."]
pub type LSTIMER0_RST_R = crate::BitReader<bool>;
#[doc = "Field `LSTIMER0_RST` writer - reg_lstimer0_rst."]
pub type LSTIMER0_RST_W<'a> = crate::BitWriter<'a, u32, LSTIMER0_CONF_SPEC, bool, 23>;
#[doc = "Field `TICK_SEL_LSTIMER0` reader - reg_tick_sel_lstimer0."]
pub type TICK_SEL_LSTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `TICK_SEL_LSTIMER0` writer - reg_tick_sel_lstimer0."]
pub type TICK_SEL_LSTIMER0_W<'a> = crate::BitWriter<'a, u32, LSTIMER0_CONF_SPEC, bool, 24>;
#[doc = "Field `LSTIMER0_PARA_UP` writer - reg_lstimer0_para_up."]
pub type LSTIMER0_PARA_UP_W<'a> = crate::BitWriter<'a, u32, LSTIMER0_CONF_SPEC, bool, 25>;
impl R {
    #[doc = "Bits 0:3 - reg_lstimer0_duty_res."]
    #[inline(always)]
    pub fn lstimer0_duty_res(&self) -> LSTIMER0_DUTY_RES_R {
        LSTIMER0_DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer0."]
    #[inline(always)]
    pub fn clk_div_lstimer0(&self) -> CLK_DIV_LSTIMER0_R {
        CLK_DIV_LSTIMER0_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 22 - reg_lstimer0_pause."]
    #[inline(always)]
    pub fn lstimer0_pause(&self) -> LSTIMER0_PAUSE_R {
        LSTIMER0_PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_lstimer0_rst."]
    #[inline(always)]
    pub fn lstimer0_rst(&self) -> LSTIMER0_RST_R {
        LSTIMER0_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer0."]
    #[inline(always)]
    pub fn tick_sel_lstimer0(&self) -> TICK_SEL_LSTIMER0_R {
        TICK_SEL_LSTIMER0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_lstimer0_duty_res."]
    #[inline(always)]
    pub fn lstimer0_duty_res(&mut self) -> LSTIMER0_DUTY_RES_W {
        LSTIMER0_DUTY_RES_W::new(self)
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer0."]
    #[inline(always)]
    pub fn clk_div_lstimer0(&mut self) -> CLK_DIV_LSTIMER0_W {
        CLK_DIV_LSTIMER0_W::new(self)
    }
    #[doc = "Bit 22 - reg_lstimer0_pause."]
    #[inline(always)]
    pub fn lstimer0_pause(&mut self) -> LSTIMER0_PAUSE_W {
        LSTIMER0_PAUSE_W::new(self)
    }
    #[doc = "Bit 23 - reg_lstimer0_rst."]
    #[inline(always)]
    pub fn lstimer0_rst(&mut self) -> LSTIMER0_RST_W {
        LSTIMER0_RST_W::new(self)
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer0."]
    #[inline(always)]
    pub fn tick_sel_lstimer0(&mut self) -> TICK_SEL_LSTIMER0_W {
        TICK_SEL_LSTIMER0_W::new(self)
    }
    #[doc = "Bit 25 - reg_lstimer0_para_up."]
    #[inline(always)]
    pub fn lstimer0_para_up(&mut self) -> LSTIMER0_PARA_UP_W {
        LSTIMER0_PARA_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSTIMER0_CONF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer0_conf](index.html) module"]
pub struct LSTIMER0_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer0_conf::R](R) reader structure"]
impl crate::Readable for LSTIMER0_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstimer0_conf::W](W) writer structure"]
impl crate::Writable for LSTIMER0_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSTIMER0_CONF to value 0x0080_0000"]
impl crate::Resettable for LSTIMER0_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
