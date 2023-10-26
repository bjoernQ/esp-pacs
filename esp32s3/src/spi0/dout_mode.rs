#[doc = "Register `DOUT_MODE` reader"]
pub type R = crate::R<DOUT_MODE_SPEC>;
#[doc = "Register `DOUT_MODE` writer"]
pub type W = crate::W<DOUT_MODE_SPEC>;
#[doc = "Field `DOUT0_MODE` reader - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT0_MODE_R = crate::BitReader;
#[doc = "Field `DOUT0_MODE` writer - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT0_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUT1_MODE` reader - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT1_MODE_R = crate::BitReader;
#[doc = "Field `DOUT1_MODE` writer - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT1_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUT2_MODE` reader - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT2_MODE_R = crate::BitReader;
#[doc = "Field `DOUT2_MODE` writer - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT2_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUT3_MODE` reader - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT3_MODE_R = crate::BitReader;
#[doc = "Field `DOUT3_MODE` writer - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT3_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUT4_MODE` reader - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT4_MODE_R = crate::BitReader;
#[doc = "Field `DOUT4_MODE` writer - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT4_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUT5_MODE` reader - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT5_MODE_R = crate::BitReader;
#[doc = "Field `DOUT5_MODE` writer - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT5_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUT6_MODE` reader - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT6_MODE_R = crate::BitReader;
#[doc = "Field `DOUT6_MODE` writer - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT6_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUT7_MODE` reader - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT7_MODE_R = crate::BitReader;
#[doc = "Field `DOUT7_MODE` writer - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUT7_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUTS_MODE` reader - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUTS_MODE_R = crate::BitReader;
#[doc = "Field `DOUTS_MODE` writer - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub type DOUTS_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT4_MODE_R {
        DOUT4_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT5_MODE_R {
        DOUT5_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT6_MODE_R {
        DOUT6_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT7_MODE_R {
        DOUT7_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn douts_mode(&self) -> DOUTS_MODE_R {
        DOUTS_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT_MODE")
            .field("dout0_mode", &format_args!("{}", self.dout0_mode().bit()))
            .field("dout1_mode", &format_args!("{}", self.dout1_mode().bit()))
            .field("dout2_mode", &format_args!("{}", self.dout2_mode().bit()))
            .field("dout3_mode", &format_args!("{}", self.dout3_mode().bit()))
            .field("dout4_mode", &format_args!("{}", self.dout4_mode().bit()))
            .field("dout5_mode", &format_args!("{}", self.dout5_mode().bit()))
            .field("dout6_mode", &format_args!("{}", self.dout6_mode().bit()))
            .field("dout7_mode", &format_args!("{}", self.dout7_mode().bit()))
            .field("douts_mode", &format_args!("{}", self.douts_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOUT_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout0_mode(&mut self) -> DOUT0_MODE_W<DOUT_MODE_SPEC, 0> {
        DOUT0_MODE_W::new(self)
    }
    #[doc = "Bit 1 - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout1_mode(&mut self) -> DOUT1_MODE_W<DOUT_MODE_SPEC, 1> {
        DOUT1_MODE_W::new(self)
    }
    #[doc = "Bit 2 - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout2_mode(&mut self) -> DOUT2_MODE_W<DOUT_MODE_SPEC, 2> {
        DOUT2_MODE_W::new(self)
    }
    #[doc = "Bit 3 - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout3_mode(&mut self) -> DOUT3_MODE_W<DOUT_MODE_SPEC, 3> {
        DOUT3_MODE_W::new(self)
    }
    #[doc = "Bit 4 - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout4_mode(&mut self) -> DOUT4_MODE_W<DOUT_MODE_SPEC, 4> {
        DOUT4_MODE_W::new(self)
    }
    #[doc = "Bit 5 - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout5_mode(&mut self) -> DOUT5_MODE_W<DOUT_MODE_SPEC, 5> {
        DOUT5_MODE_W::new(self)
    }
    #[doc = "Bit 6 - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout6_mode(&mut self) -> DOUT6_MODE_W<DOUT_MODE_SPEC, 6> {
        DOUT6_MODE_W::new(self)
    }
    #[doc = "Bit 7 - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn dout7_mode(&mut self) -> DOUT7_MODE_W<DOUT_MODE_SPEC, 7> {
        DOUT7_MODE_W::new(self)
    }
    #[doc = "Bit 8 - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    #[must_use]
    pub fn douts_mode(&mut self) -> DOUTS_MODE_W<DOUT_MODE_SPEC, 8> {
        DOUTS_MODE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MSPI output timing delay mode control register when accesses to flash.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUT_MODE_SPEC;
impl crate::RegisterSpec for DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_mode::R`](R) reader structure"]
impl crate::Readable for DOUT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dout_mode::W`](W) writer structure"]
impl crate::Writable for DOUT_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DOUT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
