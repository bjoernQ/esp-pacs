#[doc = "Register `HSTIMER0_CONF` reader"]
pub struct R(crate::R<HSTIMER0_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTIMER0_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTIMER0_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTIMER0_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTIMER0_CONF` writer"]
pub struct W(crate::W<HSTIMER0_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTIMER0_CONF_SPEC>;
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
impl From<crate::W<HSTIMER0_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTIMER0_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTIMER0_DUTY_RES` reader - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
pub type HSTIMER0_DUTY_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTIMER0_DUTY_RES` writer - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
pub type HSTIMER0_DUTY_RES_W<'a> = crate::FieldWriter<'a, u32, HSTIMER0_CONF_SPEC, u8, u8, 5, 0>;
#[doc = "Field `DIV_NUM_HSTIMER0` reader - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_HSTIMER0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIV_NUM_HSTIMER0` writer - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_HSTIMER0_W<'a> = crate::FieldWriter<'a, u32, HSTIMER0_CONF_SPEC, u32, u32, 18, 5>;
#[doc = "Field `HSTIMER0_PAUSE` reader - This bit is used to pause the counter in high speed timer0"]
pub type HSTIMER0_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `HSTIMER0_PAUSE` writer - This bit is used to pause the counter in high speed timer0"]
pub type HSTIMER0_PAUSE_W<'a> = crate::BitWriter<'a, u32, HSTIMER0_CONF_SPEC, bool, 23>;
#[doc = "Field `HSTIMER0_RST` reader - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
pub type HSTIMER0_RST_R = crate::BitReader<bool>;
#[doc = "Field `HSTIMER0_RST` writer - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
pub type HSTIMER0_RST_W<'a> = crate::BitWriter<'a, u32, HSTIMER0_CONF_SPEC, bool, 24>;
#[doc = "Field `TICK_SEL_HSTIMER0` reader - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
pub type TICK_SEL_HSTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `TICK_SEL_HSTIMER0` writer - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
pub type TICK_SEL_HSTIMER0_W<'a> = crate::BitWriter<'a, u32, HSTIMER0_CONF_SPEC, bool, 25>;
#[doc = "Field `HSTIMER0_LIM` reader - "]
pub type HSTIMER0_LIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTIMER0_LIM` writer - "]
pub type HSTIMER0_LIM_W<'a> = crate::FieldWriter<'a, u32, HSTIMER0_CONF_SPEC, u8, u8, 5, 31>;
impl R {
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer0_duty_res(&self) -> HSTIMER0_DUTY_RES_R {
        HSTIMER0_DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer0(&self) -> DIV_NUM_HSTIMER0_R {
        DIV_NUM_HSTIMER0_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer0"]
    #[inline(always)]
    pub fn hstimer0_pause(&self) -> HSTIMER0_PAUSE_R {
        HSTIMER0_PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer0_rst(&self) -> HSTIMER0_RST_R {
        HSTIMER0_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer0(&self) -> TICK_SEL_HSTIMER0_R {
        TICK_SEL_HSTIMER0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn hstimer0_lim(&self) -> HSTIMER0_LIM_R {
        HSTIMER0_LIM_R::new(((self.bits >> 31) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer0_duty_res(&mut self) -> HSTIMER0_DUTY_RES_W {
        HSTIMER0_DUTY_RES_W::new(self)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer0(&mut self) -> DIV_NUM_HSTIMER0_W {
        DIV_NUM_HSTIMER0_W::new(self)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer0"]
    #[inline(always)]
    pub fn hstimer0_pause(&mut self) -> HSTIMER0_PAUSE_W {
        HSTIMER0_PAUSE_W::new(self)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer0_rst(&mut self) -> HSTIMER0_RST_W {
        HSTIMER0_RST_W::new(self)
    }
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer0(&mut self) -> TICK_SEL_HSTIMER0_W {
        TICK_SEL_HSTIMER0_W::new(self)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn hstimer0_lim(&mut self) -> HSTIMER0_LIM_W {
        HSTIMER0_LIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstimer0_conf](index.html) module"]
pub struct HSTIMER0_CONF_SPEC;
impl crate::RegisterSpec for HSTIMER0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstimer0_conf::R](R) reader structure"]
impl crate::Readable for HSTIMER0_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstimer0_conf::W](W) writer structure"]
impl crate::Writable for HSTIMER0_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTIMER0_CONF to value 0x0100_0000"]
impl crate::Resettable for HSTIMER0_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
