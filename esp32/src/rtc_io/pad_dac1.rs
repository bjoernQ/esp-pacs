#[doc = "Register `PAD_DAC1` reader"]
pub type R = crate::R<PAD_DAC1_SPEC>;
#[doc = "Register `PAD_DAC1` writer"]
pub type W = crate::W<PAD_DAC1_SPEC>;
#[doc = "Field `PDAC1_DAC_XPD_FORCE` reader - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC1_DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC1_DAC_XPD_FORCE` writer - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC1_DAC_XPD_FORCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_FUN_IE` reader - the input enable of the pad"]
pub type PDAC1_FUN_IE_R = crate::BitReader;
#[doc = "Field `PDAC1_FUN_IE` writer - the input enable of the pad"]
pub type PDAC1_FUN_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_SLP_OE` reader - the output enable of the pad in sleep status"]
pub type PDAC1_SLP_OE_R = crate::BitReader;
#[doc = "Field `PDAC1_SLP_OE` writer - the output enable of the pad in sleep status"]
pub type PDAC1_SLP_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type PDAC1_SLP_IE_R = crate::BitReader;
#[doc = "Field `PDAC1_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type PDAC1_SLP_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type PDAC1_SLP_SEL_R = crate::BitReader;
#[doc = "Field `PDAC1_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type PDAC1_SLP_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_FUN_SEL` reader - the functional selection signal of the pad"]
pub type PDAC1_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `PDAC1_FUN_SEL` writer - the functional selection signal of the pad"]
pub type PDAC1_FUN_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PDAC1_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type PDAC1_MUX_SEL_R = crate::BitReader;
#[doc = "Field `PDAC1_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type PDAC1_MUX_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_XPD_DAC` reader - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC1_XPD_DAC_R = crate::BitReader;
#[doc = "Field `PDAC1_XPD_DAC` writer - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC1_XPD_DAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_DAC` reader - PAD DAC1 control code."]
pub type PDAC1_DAC_R = crate::FieldReader;
#[doc = "Field `PDAC1_DAC` writer - PAD DAC1 control code."]
pub type PDAC1_DAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PDAC1_RUE` reader - the pull up enable of the pad"]
pub type PDAC1_RUE_R = crate::BitReader;
#[doc = "Field `PDAC1_RUE` writer - the pull up enable of the pad"]
pub type PDAC1_RUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_RDE` reader - the pull down enable of the pad"]
pub type PDAC1_RDE_R = crate::BitReader;
#[doc = "Field `PDAC1_RDE` writer - the pull down enable of the pad"]
pub type PDAC1_RDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type PDAC1_HOLD_R = crate::BitReader;
#[doc = "Field `PDAC1_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type PDAC1_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDAC1_DRV` reader - the driver strength of the pad"]
pub type PDAC1_DRV_R = crate::FieldReader;
#[doc = "Field `PDAC1_DRV` writer - the driver strength of the pad"]
pub type PDAC1_DRV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac1_dac_xpd_force(&self) -> PDAC1_DAC_XPD_FORCE_R {
        PDAC1_DAC_XPD_FORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn pdac1_fun_ie(&self) -> PDAC1_FUN_IE_R {
        PDAC1_FUN_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac1_slp_oe(&self) -> PDAC1_SLP_OE_R {
        PDAC1_SLP_OE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac1_slp_ie(&self) -> PDAC1_SLP_IE_R {
        PDAC1_SLP_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn pdac1_slp_sel(&self) -> PDAC1_SLP_SEL_R {
        PDAC1_SLP_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn pdac1_fun_sel(&self) -> PDAC1_FUN_SEL_R {
        PDAC1_FUN_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn pdac1_mux_sel(&self) -> PDAC1_MUX_SEL_R {
        PDAC1_MUX_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac1_xpd_dac(&self) -> PDAC1_XPD_DAC_R {
        PDAC1_XPD_DAC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - PAD DAC1 control code."]
    #[inline(always)]
    pub fn pdac1_dac(&self) -> PDAC1_DAC_R {
        PDAC1_DAC_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn pdac1_rue(&self) -> PDAC1_RUE_R {
        PDAC1_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn pdac1_rde(&self) -> PDAC1_RDE_R {
        PDAC1_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn pdac1_hold(&self) -> PDAC1_HOLD_R {
        PDAC1_HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn pdac1_drv(&self) -> PDAC1_DRV_R {
        PDAC1_DRV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC1")
            .field(
                "pdac1_dac_xpd_force",
                &format_args!("{}", self.pdac1_dac_xpd_force().bit()),
            )
            .field(
                "pdac1_fun_ie",
                &format_args!("{}", self.pdac1_fun_ie().bit()),
            )
            .field(
                "pdac1_slp_oe",
                &format_args!("{}", self.pdac1_slp_oe().bit()),
            )
            .field(
                "pdac1_slp_ie",
                &format_args!("{}", self.pdac1_slp_ie().bit()),
            )
            .field(
                "pdac1_slp_sel",
                &format_args!("{}", self.pdac1_slp_sel().bit()),
            )
            .field(
                "pdac1_fun_sel",
                &format_args!("{}", self.pdac1_fun_sel().bits()),
            )
            .field(
                "pdac1_mux_sel",
                &format_args!("{}", self.pdac1_mux_sel().bit()),
            )
            .field(
                "pdac1_xpd_dac",
                &format_args!("{}", self.pdac1_xpd_dac().bit()),
            )
            .field("pdac1_dac", &format_args!("{}", self.pdac1_dac().bits()))
            .field("pdac1_rue", &format_args!("{}", self.pdac1_rue().bit()))
            .field("pdac1_rde", &format_args!("{}", self.pdac1_rde().bit()))
            .field("pdac1_hold", &format_args!("{}", self.pdac1_hold().bit()))
            .field("pdac1_drv", &format_args!("{}", self.pdac1_drv().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD_DAC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_dac_xpd_force(&mut self) -> PDAC1_DAC_XPD_FORCE_W<PAD_DAC1_SPEC, 10> {
        PDAC1_DAC_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_fun_ie(&mut self) -> PDAC1_FUN_IE_W<PAD_DAC1_SPEC, 11> {
        PDAC1_FUN_IE_W::new(self)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_slp_oe(&mut self) -> PDAC1_SLP_OE_W<PAD_DAC1_SPEC, 12> {
        PDAC1_SLP_OE_W::new(self)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_slp_ie(&mut self) -> PDAC1_SLP_IE_W<PAD_DAC1_SPEC, 13> {
        PDAC1_SLP_IE_W::new(self)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_slp_sel(&mut self) -> PDAC1_SLP_SEL_W<PAD_DAC1_SPEC, 14> {
        PDAC1_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_fun_sel(&mut self) -> PDAC1_FUN_SEL_W<PAD_DAC1_SPEC, 15> {
        PDAC1_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_mux_sel(&mut self) -> PDAC1_MUX_SEL_W<PAD_DAC1_SPEC, 17> {
        PDAC1_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_xpd_dac(&mut self) -> PDAC1_XPD_DAC_W<PAD_DAC1_SPEC, 18> {
        PDAC1_XPD_DAC_W::new(self)
    }
    #[doc = "Bits 19:26 - PAD DAC1 control code."]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_dac(&mut self) -> PDAC1_DAC_W<PAD_DAC1_SPEC, 19> {
        PDAC1_DAC_W::new(self)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_rue(&mut self) -> PDAC1_RUE_W<PAD_DAC1_SPEC, 27> {
        PDAC1_RUE_W::new(self)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_rde(&mut self) -> PDAC1_RDE_W<PAD_DAC1_SPEC, 28> {
        PDAC1_RDE_W::new(self)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_hold(&mut self) -> PDAC1_HOLD_W<PAD_DAC1_SPEC, 29> {
        PDAC1_HOLD_W::new(self)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_drv(&mut self) -> PDAC1_DRV_W<PAD_DAC1_SPEC, 30> {
        PDAC1_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_dac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_DAC1_SPEC;
impl crate::RegisterSpec for PAD_DAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_dac1::R`](R) reader structure"]
impl crate::Readable for PAD_DAC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_dac1::W`](W) writer structure"]
impl crate::Writable for PAD_DAC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_DAC1 to value 0x8000_0000"]
impl crate::Resettable for PAD_DAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
