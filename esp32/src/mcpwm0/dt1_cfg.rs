#[doc = "Register `DT1_CFG` reader"]
pub type R = crate::R<DT1_CFG_SPEC>;
#[doc = "Register `DT1_CFG` writer"]
pub type W = crate::W<DT1_CFG_SPEC>;
#[doc = "Field `DT1_FED_UPMETHOD` reader - "]
pub type DT1_FED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DT1_FED_UPMETHOD` writer - "]
pub type DT1_FED_UPMETHOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DT1_RED_UPMETHOD` reader - "]
pub type DT1_RED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DT1_RED_UPMETHOD` writer - "]
pub type DT1_RED_UPMETHOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DT1_DEB_MODE` reader - "]
pub type DT1_DEB_MODE_R = crate::BitReader;
#[doc = "Field `DT1_DEB_MODE` writer - "]
pub type DT1_DEB_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_A_OUTSWAP` reader - "]
pub type DT1_A_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DT1_A_OUTSWAP` writer - "]
pub type DT1_A_OUTSWAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_B_OUTSWAP` reader - "]
pub type DT1_B_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DT1_B_OUTSWAP` writer - "]
pub type DT1_B_OUTSWAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_RED_INSEL` reader - "]
pub type DT1_RED_INSEL_R = crate::BitReader;
#[doc = "Field `DT1_RED_INSEL` writer - "]
pub type DT1_RED_INSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_FED_INSEL` reader - "]
pub type DT1_FED_INSEL_R = crate::BitReader;
#[doc = "Field `DT1_FED_INSEL` writer - "]
pub type DT1_FED_INSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_RED_OUTINVERT` reader - "]
pub type DT1_RED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DT1_RED_OUTINVERT` writer - "]
pub type DT1_RED_OUTINVERT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_FED_OUTINVERT` reader - "]
pub type DT1_FED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DT1_FED_OUTINVERT` writer - "]
pub type DT1_FED_OUTINVERT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_A_OUTBYPASS` reader - "]
pub type DT1_A_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DT1_A_OUTBYPASS` writer - "]
pub type DT1_A_OUTBYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_B_OUTBYPASS` reader - "]
pub type DT1_B_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DT1_B_OUTBYPASS` writer - "]
pub type DT1_B_OUTBYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DT1_CLK_SEL` reader - "]
pub type DT1_CLK_SEL_R = crate::BitReader;
#[doc = "Field `DT1_CLK_SEL` writer - "]
pub type DT1_CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dt1_fed_upmethod(&self) -> DT1_FED_UPMETHOD_R {
        DT1_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dt1_red_upmethod(&self) -> DT1_RED_UPMETHOD_R {
        DT1_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dt1_deb_mode(&self) -> DT1_DEB_MODE_R {
        DT1_DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dt1_a_outswap(&self) -> DT1_A_OUTSWAP_R {
        DT1_A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dt1_b_outswap(&self) -> DT1_B_OUTSWAP_R {
        DT1_B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dt1_red_insel(&self) -> DT1_RED_INSEL_R {
        DT1_RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dt1_fed_insel(&self) -> DT1_FED_INSEL_R {
        DT1_FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dt1_red_outinvert(&self) -> DT1_RED_OUTINVERT_R {
        DT1_RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dt1_fed_outinvert(&self) -> DT1_FED_OUTINVERT_R {
        DT1_FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dt1_a_outbypass(&self) -> DT1_A_OUTBYPASS_R {
        DT1_A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dt1_b_outbypass(&self) -> DT1_B_OUTBYPASS_R {
        DT1_B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dt1_clk_sel(&self) -> DT1_CLK_SEL_R {
        DT1_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT1_CFG")
            .field(
                "dt1_fed_upmethod",
                &format_args!("{}", self.dt1_fed_upmethod().bits()),
            )
            .field(
                "dt1_red_upmethod",
                &format_args!("{}", self.dt1_red_upmethod().bits()),
            )
            .field(
                "dt1_deb_mode",
                &format_args!("{}", self.dt1_deb_mode().bit()),
            )
            .field(
                "dt1_a_outswap",
                &format_args!("{}", self.dt1_a_outswap().bit()),
            )
            .field(
                "dt1_b_outswap",
                &format_args!("{}", self.dt1_b_outswap().bit()),
            )
            .field(
                "dt1_red_insel",
                &format_args!("{}", self.dt1_red_insel().bit()),
            )
            .field(
                "dt1_fed_insel",
                &format_args!("{}", self.dt1_fed_insel().bit()),
            )
            .field(
                "dt1_red_outinvert",
                &format_args!("{}", self.dt1_red_outinvert().bit()),
            )
            .field(
                "dt1_fed_outinvert",
                &format_args!("{}", self.dt1_fed_outinvert().bit()),
            )
            .field(
                "dt1_a_outbypass",
                &format_args!("{}", self.dt1_a_outbypass().bit()),
            )
            .field(
                "dt1_b_outbypass",
                &format_args!("{}", self.dt1_b_outbypass().bit()),
            )
            .field("dt1_clk_sel", &format_args!("{}", self.dt1_clk_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DT1_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_fed_upmethod(&mut self) -> DT1_FED_UPMETHOD_W<DT1_CFG_SPEC, 0> {
        DT1_FED_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_red_upmethod(&mut self) -> DT1_RED_UPMETHOD_W<DT1_CFG_SPEC, 4> {
        DT1_RED_UPMETHOD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_deb_mode(&mut self) -> DT1_DEB_MODE_W<DT1_CFG_SPEC, 8> {
        DT1_DEB_MODE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_a_outswap(&mut self) -> DT1_A_OUTSWAP_W<DT1_CFG_SPEC, 9> {
        DT1_A_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_b_outswap(&mut self) -> DT1_B_OUTSWAP_W<DT1_CFG_SPEC, 10> {
        DT1_B_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_red_insel(&mut self) -> DT1_RED_INSEL_W<DT1_CFG_SPEC, 11> {
        DT1_RED_INSEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_fed_insel(&mut self) -> DT1_FED_INSEL_W<DT1_CFG_SPEC, 12> {
        DT1_FED_INSEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_red_outinvert(&mut self) -> DT1_RED_OUTINVERT_W<DT1_CFG_SPEC, 13> {
        DT1_RED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_fed_outinvert(&mut self) -> DT1_FED_OUTINVERT_W<DT1_CFG_SPEC, 14> {
        DT1_FED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_a_outbypass(&mut self) -> DT1_A_OUTBYPASS_W<DT1_CFG_SPEC, 15> {
        DT1_A_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_b_outbypass(&mut self) -> DT1_B_OUTBYPASS_W<DT1_CFG_SPEC, 16> {
        DT1_B_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_clk_sel(&mut self) -> DT1_CLK_SEL_W<DT1_CFG_SPEC, 17> {
        DT1_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT1_CFG_SPEC;
impl crate::RegisterSpec for DT1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt1_cfg::R`](R) reader structure"]
impl crate::Readable for DT1_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt1_cfg::W`](W) writer structure"]
impl crate::Writable for DT1_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT1_CFG to value 0x0001_8000"]
impl crate::Resettable for DT1_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_8000;
}
