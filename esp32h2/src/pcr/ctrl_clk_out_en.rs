#[doc = "Register `CTRL_CLK_OUT_EN` reader"]
pub type R = crate::R<CTRL_CLK_OUT_EN_SPEC>;
#[doc = "Register `CTRL_CLK_OUT_EN` writer"]
pub type W = crate::W<CTRL_CLK_OUT_EN_SPEC>;
#[doc = "Field `CLK8_OEN` reader - Set 1 to enable 8m clock"]
pub type CLK8_OEN_R = crate::BitReader;
#[doc = "Field `CLK8_OEN` writer - Set 1 to enable 8m clock"]
pub type CLK8_OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK16_OEN` reader - Set 1 to enable 16m clock"]
pub type CLK16_OEN_R = crate::BitReader;
#[doc = "Field `CLK16_OEN` writer - Set 1 to enable 16m clock"]
pub type CLK16_OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK32_OEN` reader - Set 1 to enable 32m clock"]
pub type CLK32_OEN_R = crate::BitReader;
#[doc = "Field `CLK32_OEN` writer - Set 1 to enable 32m clock"]
pub type CLK32_OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_ADC_INF_OEN` reader - Reserved"]
pub type CLK_ADC_INF_OEN_R = crate::BitReader;
#[doc = "Field `CLK_ADC_INF_OEN` writer - Reserved"]
pub type CLK_ADC_INF_OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_DFM_INF_OEN` reader - Reserved"]
pub type CLK_DFM_INF_OEN_R = crate::BitReader;
#[doc = "Field `CLK_DFM_INF_OEN` writer - Reserved"]
pub type CLK_DFM_INF_OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_SDM_MOD_OEN` reader - Reserved"]
pub type CLK_SDM_MOD_OEN_R = crate::BitReader;
#[doc = "Field `CLK_SDM_MOD_OEN` writer - Reserved"]
pub type CLK_SDM_MOD_OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_XTAL_OEN` reader - Set 1 to enable xtal clock"]
pub type CLK_XTAL_OEN_R = crate::BitReader;
#[doc = "Field `CLK_XTAL_OEN` writer - Set 1 to enable xtal clock"]
pub type CLK_XTAL_OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable 8m clock"]
    #[inline(always)]
    pub fn clk8_oen(&self) -> CLK8_OEN_R {
        CLK8_OEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable 16m clock"]
    #[inline(always)]
    pub fn clk16_oen(&self) -> CLK16_OEN_R {
        CLK16_OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable 32m clock"]
    #[inline(always)]
    pub fn clk32_oen(&self) -> CLK32_OEN_R {
        CLK32_OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn clk_adc_inf_oen(&self) -> CLK_ADC_INF_OEN_R {
        CLK_ADC_INF_OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn clk_dfm_inf_oen(&self) -> CLK_DFM_INF_OEN_R {
        CLK_DFM_INF_OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn clk_sdm_mod_oen(&self) -> CLK_SDM_MOD_OEN_R {
        CLK_SDM_MOD_OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to enable xtal clock"]
    #[inline(always)]
    pub fn clk_xtal_oen(&self) -> CLK_XTAL_OEN_R {
        CLK_XTAL_OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_CLK_OUT_EN")
            .field("clk8_oen", &format_args!("{}", self.clk8_oen().bit()))
            .field("clk16_oen", &format_args!("{}", self.clk16_oen().bit()))
            .field("clk32_oen", &format_args!("{}", self.clk32_oen().bit()))
            .field(
                "clk_adc_inf_oen",
                &format_args!("{}", self.clk_adc_inf_oen().bit()),
            )
            .field(
                "clk_dfm_inf_oen",
                &format_args!("{}", self.clk_dfm_inf_oen().bit()),
            )
            .field(
                "clk_sdm_mod_oen",
                &format_args!("{}", self.clk_sdm_mod_oen().bit()),
            )
            .field(
                "clk_xtal_oen",
                &format_args!("{}", self.clk_xtal_oen().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_CLK_OUT_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable 8m clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk8_oen(&mut self) -> CLK8_OEN_W<CTRL_CLK_OUT_EN_SPEC, 0> {
        CLK8_OEN_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to enable 16m clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk16_oen(&mut self) -> CLK16_OEN_W<CTRL_CLK_OUT_EN_SPEC, 1> {
        CLK16_OEN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable 32m clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk32_oen(&mut self) -> CLK32_OEN_W<CTRL_CLK_OUT_EN_SPEC, 2> {
        CLK32_OEN_W::new(self)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clk_adc_inf_oen(&mut self) -> CLK_ADC_INF_OEN_W<CTRL_CLK_OUT_EN_SPEC, 3> {
        CLK_ADC_INF_OEN_W::new(self)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dfm_inf_oen(&mut self) -> CLK_DFM_INF_OEN_W<CTRL_CLK_OUT_EN_SPEC, 4> {
        CLK_DFM_INF_OEN_W::new(self)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdm_mod_oen(&mut self) -> CLK_SDM_MOD_OEN_W<CTRL_CLK_OUT_EN_SPEC, 5> {
        CLK_SDM_MOD_OEN_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to enable xtal clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_xtal_oen(&mut self) -> CLK_XTAL_OEN_W<CTRL_CLK_OUT_EN_SPEC, 6> {
        CLK_XTAL_OEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CLK_OUT_EN configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_clk_out_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_clk_out_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_CLK_OUT_EN_SPEC;
impl crate::RegisterSpec for CTRL_CLK_OUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_clk_out_en::R`](R) reader structure"]
impl crate::Readable for CTRL_CLK_OUT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_clk_out_en::W`](W) writer structure"]
impl crate::Writable for CTRL_CLK_OUT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_CLK_OUT_EN to value 0x7f"]
impl crate::Resettable for CTRL_CLK_OUT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
