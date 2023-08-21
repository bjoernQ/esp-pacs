#[doc = "Register `SARADC_CLKM_CONF` reader"]
pub type R = crate::R<SARADC_CLKM_CONF_SPEC>;
#[doc = "Register `SARADC_CLKM_CONF` writer"]
pub type W = crate::W<SARADC_CLKM_CONF_SPEC>;
#[doc = "Field `SARADC_CLKM_DIV_A` reader - The denominator of the frequency divider factor of the saradc function clock."]
pub type SARADC_CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `SARADC_CLKM_DIV_A` writer - The denominator of the frequency divider factor of the saradc function clock."]
pub type SARADC_CLKM_DIV_A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SARADC_CLKM_DIV_B` reader - The numerator of the frequency divider factor of the saradc function clock."]
pub type SARADC_CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `SARADC_CLKM_DIV_B` writer - The numerator of the frequency divider factor of the saradc function clock."]
pub type SARADC_CLKM_DIV_B_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SARADC_CLKM_DIV_NUM` reader - The integral part of the frequency divider factor of the saradc function clock."]
pub type SARADC_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SARADC_CLKM_DIV_NUM` writer - The integral part of the frequency divider factor of the saradc function clock."]
pub type SARADC_CLKM_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SARADC_CLKM_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: reserved."]
pub type SARADC_CLKM_SEL_R = crate::FieldReader;
#[doc = "Field `SARADC_CLKM_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: reserved."]
pub type SARADC_CLKM_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SARADC_CLKM_EN` reader - Set 1 to enable saradc function clock"]
pub type SARADC_CLKM_EN_R = crate::BitReader;
#[doc = "Field `SARADC_CLKM_EN` writer - Set 1 to enable saradc function clock"]
pub type SARADC_CLKM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the saradc function clock."]
    #[inline(always)]
    pub fn saradc_clkm_div_a(&self) -> SARADC_CLKM_DIV_A_R {
        SARADC_CLKM_DIV_A_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the saradc function clock."]
    #[inline(always)]
    pub fn saradc_clkm_div_b(&self) -> SARADC_CLKM_DIV_B_R {
        SARADC_CLKM_DIV_B_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the saradc function clock."]
    #[inline(always)]
    pub fn saradc_clkm_div_num(&self) -> SARADC_CLKM_DIV_NUM_R {
        SARADC_CLKM_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    pub fn saradc_clkm_sel(&self) -> SARADC_CLKM_SEL_R {
        SARADC_CLKM_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable saradc function clock"]
    #[inline(always)]
    pub fn saradc_clkm_en(&self) -> SARADC_CLKM_EN_R {
        SARADC_CLKM_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SARADC_CLKM_CONF")
            .field(
                "saradc_clkm_div_a",
                &format_args!("{}", self.saradc_clkm_div_a().bits()),
            )
            .field(
                "saradc_clkm_div_b",
                &format_args!("{}", self.saradc_clkm_div_b().bits()),
            )
            .field(
                "saradc_clkm_div_num",
                &format_args!("{}", self.saradc_clkm_div_num().bits()),
            )
            .field(
                "saradc_clkm_sel",
                &format_args!("{}", self.saradc_clkm_sel().bits()),
            )
            .field(
                "saradc_clkm_en",
                &format_args!("{}", self.saradc_clkm_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SARADC_CLKM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the saradc function clock."]
    #[inline(always)]
    #[must_use]
    pub fn saradc_clkm_div_a(&mut self) -> SARADC_CLKM_DIV_A_W<SARADC_CLKM_CONF_SPEC, 0> {
        SARADC_CLKM_DIV_A_W::new(self)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the saradc function clock."]
    #[inline(always)]
    #[must_use]
    pub fn saradc_clkm_div_b(&mut self) -> SARADC_CLKM_DIV_B_W<SARADC_CLKM_CONF_SPEC, 6> {
        SARADC_CLKM_DIV_B_W::new(self)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the saradc function clock."]
    #[inline(always)]
    #[must_use]
    pub fn saradc_clkm_div_num(&mut self) -> SARADC_CLKM_DIV_NUM_W<SARADC_CLKM_CONF_SPEC, 12> {
        SARADC_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn saradc_clkm_sel(&mut self) -> SARADC_CLKM_SEL_W<SARADC_CLKM_CONF_SPEC, 20> {
        SARADC_CLKM_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable saradc function clock"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_clkm_en(&mut self) -> SARADC_CLKM_EN_W<SARADC_CLKM_CONF_SPEC, 22> {
        SARADC_CLKM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SARADC_CLKM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_clkm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_clkm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SARADC_CLKM_CONF_SPEC;
impl crate::RegisterSpec for SARADC_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saradc_clkm_conf::R`](R) reader structure"]
impl crate::Readable for SARADC_CLKM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saradc_clkm_conf::W`](W) writer structure"]
impl crate::Writable for SARADC_CLKM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SARADC_CLKM_CONF to value 0x0040_4000"]
impl crate::Resettable for SARADC_CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_4000;
}
