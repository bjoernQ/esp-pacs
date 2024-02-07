#[doc = "Register `SPI_DIN_MODE` reader"]
pub type R = crate::R<SPI_DIN_MODE_SPEC>;
#[doc = "Register `SPI_DIN_MODE` writer"]
pub type W = crate::W<SPI_DIN_MODE_SPEC>;
#[doc = "Field `SPI_DIN0_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN0_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN0_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN1_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN1_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN1_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN1_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN2_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN2_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN2_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN2_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN3_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN3_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN3_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_TIMING_HCLK_ACTIVE` reader - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
pub type SPI_TIMING_HCLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_TIMING_HCLK_ACTIVE` writer - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
pub type SPI_TIMING_HCLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din0_mode(&self) -> SPI_DIN0_MODE_R {
        SPI_DIN0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din1_mode(&self) -> SPI_DIN1_MODE_R {
        SPI_DIN1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din2_mode(&self) -> SPI_DIN2_MODE_R {
        SPI_DIN2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din3_mode(&self) -> SPI_DIN3_MODE_R {
        SPI_DIN3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_timing_hclk_active(&self) -> SPI_TIMING_HCLK_ACTIVE_R {
        SPI_TIMING_HCLK_ACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DIN_MODE")
            .field(
                "spi_din0_mode",
                &format_args!("{}", self.spi_din0_mode().bits()),
            )
            .field(
                "spi_din1_mode",
                &format_args!("{}", self.spi_din1_mode().bits()),
            )
            .field(
                "spi_din2_mode",
                &format_args!("{}", self.spi_din2_mode().bits()),
            )
            .field(
                "spi_din3_mode",
                &format_args!("{}", self.spi_din3_mode().bits()),
            )
            .field(
                "spi_timing_hclk_active",
                &format_args!("{}", self.spi_timing_hclk_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_DIN_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din0_mode(&mut self) -> SPI_DIN0_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN0_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din1_mode(&mut self) -> SPI_DIN1_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN1_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din2_mode(&mut self) -> SPI_DIN2_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN2_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din3_mode(&mut self) -> SPI_DIN3_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN3_MODE_W::new(self, 6)
    }
    #[doc = "Bit 16 - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_timing_hclk_active(&mut self) -> SPI_TIMING_HCLK_ACTIVE_W<SPI_DIN_MODE_SPEC> {
        SPI_TIMING_HCLK_ACTIVE_W::new(self, 16)
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
#[doc = "SPI input delay mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_din_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_din_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_din_mode::R`](R) reader structure"]
impl crate::Readable for SPI_DIN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_din_mode::W`](W) writer structure"]
impl crate::Writable for SPI_DIN_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_DIN_MODE to value 0"]
impl crate::Resettable for SPI_DIN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}