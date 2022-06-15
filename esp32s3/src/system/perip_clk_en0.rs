#[doc = "Register `PERIP_CLK_EN0` reader"]
pub struct R(crate::R<PERIP_CLK_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN0` writer"]
pub struct W(crate::W<PERIP_CLK_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN0_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMERS_CLK_EN` reader - Set 1 to enable TIMERS clock"]
pub type TIMERS_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMERS_CLK_EN` writer - Set 1 to enable TIMERS clock"]
pub type TIMERS_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 0>;
#[doc = "Field `SPI01_CLK_EN` reader - Set 1 to enable SPI01 clock"]
pub type SPI01_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI01_CLK_EN` writer - Set 1 to enable SPI01 clock"]
pub type SPI01_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 1>;
#[doc = "Field `UART_CLK_EN` reader - Set 1 to enable UART clock"]
pub type UART_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_CLK_EN` writer - Set 1 to enable UART clock"]
pub type UART_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 2>;
#[doc = "Field `WDG_CLK_EN` reader - Set 1 to enable WDG clock"]
pub type WDG_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDG_CLK_EN` writer - Set 1 to enable WDG clock"]
pub type WDG_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 3>;
#[doc = "Field `I2S0_CLK_EN` reader - Set 1 to enable I2S0 clock"]
pub type I2S0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2S0_CLK_EN` writer - Set 1 to enable I2S0 clock"]
pub type I2S0_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 4>;
#[doc = "Field `UART1_CLK_EN` reader - Set 1 to enable UART1 clock"]
pub type UART1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_CLK_EN` writer - Set 1 to enable UART1 clock"]
pub type UART1_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 5>;
#[doc = "Field `SPI2_CLK_EN` reader - Set 1 to enable SPI2 clock"]
pub type SPI2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_CLK_EN` writer - Set 1 to enable SPI2 clock"]
pub type SPI2_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 6>;
#[doc = "Field `I2C_EXT0_CLK_EN` reader - Set 1 to enable I2C_EXT0 clock"]
pub type I2C_EXT0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EXT0_CLK_EN` writer - Set 1 to enable I2C_EXT0 clock"]
pub type I2C_EXT0_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 7>;
#[doc = "Field `UHCI0_CLK_EN` reader - Set 1 to enable UHCI0 clock"]
pub type UHCI0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UHCI0_CLK_EN` writer - Set 1 to enable UHCI0 clock"]
pub type UHCI0_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 8>;
#[doc = "Field `RMT_CLK_EN` reader - Set 1 to enable RMT clock"]
pub type RMT_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `RMT_CLK_EN` writer - Set 1 to enable RMT clock"]
pub type RMT_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 9>;
#[doc = "Field `PCNT_CLK_EN` reader - Set 1 to enable PCNT clock"]
pub type PCNT_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PCNT_CLK_EN` writer - Set 1 to enable PCNT clock"]
pub type PCNT_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 10>;
#[doc = "Field `LEDC_CLK_EN` reader - Set 1 to enable LEDC clock"]
pub type LEDC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_CLK_EN` writer - Set 1 to enable LEDC clock"]
pub type LEDC_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 11>;
#[doc = "Field `UHCI1_CLK_EN` reader - Set 1 to enable UHCI1 clock"]
pub type UHCI1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UHCI1_CLK_EN` writer - Set 1 to enable UHCI1 clock"]
pub type UHCI1_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 12>;
#[doc = "Field `TIMERGROUP_CLK_EN` reader - Set 1 to enable TIMERGROUP clock"]
pub type TIMERGROUP_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP_CLK_EN` writer - Set 1 to enable TIMERGROUP clock"]
pub type TIMERGROUP_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 13>;
#[doc = "Field `EFUSE_CLK_EN` reader - Set 1 to enable EFUSE clock"]
pub type EFUSE_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_CLK_EN` writer - Set 1 to enable EFUSE clock"]
pub type EFUSE_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 14>;
#[doc = "Field `TIMERGROUP1_CLK_EN` reader - Set 1 to enable TIMERGROUP1 clock"]
pub type TIMERGROUP1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP1_CLK_EN` writer - Set 1 to enable TIMERGROUP1 clock"]
pub type TIMERGROUP1_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 15>;
#[doc = "Field `SPI3_CLK_EN` reader - Set 1 to enable SPI3 clock"]
pub type SPI3_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_CLK_EN` writer - Set 1 to enable SPI3 clock"]
pub type SPI3_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 16>;
#[doc = "Field `PWM0_CLK_EN` reader - Set 1 to enable PWM0 clock"]
pub type PWM0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_CLK_EN` writer - Set 1 to enable PWM0 clock"]
pub type PWM0_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 17>;
#[doc = "Field `I2C_EXT1_CLK_EN` reader - Set 1 to enable I2C_EXT1 clock"]
pub type I2C_EXT1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EXT1_CLK_EN` writer - Set 1 to enable I2C_EXT1 clock"]
pub type I2C_EXT1_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 18>;
#[doc = "Field `CAN_CLK_EN` reader - Set 1 to enable CAN clock"]
pub type CAN_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CLK_EN` writer - Set 1 to enable CAN clock"]
pub type CAN_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 19>;
#[doc = "Field `PWM1_CLK_EN` reader - Set 1 to enable PWM1 clock"]
pub type PWM1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM1_CLK_EN` writer - Set 1 to enable PWM1 clock"]
pub type PWM1_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 20>;
#[doc = "Field `I2S1_CLK_EN` reader - Set 1 to enable I2S1 clock"]
pub type I2S1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2S1_CLK_EN` writer - Set 1 to enable I2S1 clock"]
pub type I2S1_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 21>;
#[doc = "Field `SPI2_DMA_CLK_EN` reader - Set 1 to enable SPI2_DMA clock"]
pub type SPI2_DMA_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_DMA_CLK_EN` writer - Set 1 to enable SPI2_DMA clock"]
pub type SPI2_DMA_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 22>;
#[doc = "Field `USB_CLK_EN` reader - Set 1 to enable USB clock"]
pub type USB_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_CLK_EN` writer - Set 1 to enable USB clock"]
pub type USB_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 23>;
#[doc = "Field `UART_MEM_CLK_EN` reader - Set 1 to enable UART_MEM clock"]
pub type UART_MEM_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_MEM_CLK_EN` writer - Set 1 to enable UART_MEM clock"]
pub type UART_MEM_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 24>;
#[doc = "Field `PWM2_CLK_EN` reader - Set 1 to enable PWM2 clock"]
pub type PWM2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM2_CLK_EN` writer - Set 1 to enable PWM2 clock"]
pub type PWM2_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 25>;
#[doc = "Field `PWM3_CLK_EN` reader - Set 1 to enable PWM3 clock"]
pub type PWM3_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM3_CLK_EN` writer - Set 1 to enable PWM3 clock"]
pub type PWM3_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 26>;
#[doc = "Field `SPI3_DMA_CLK_EN` reader - Set 1 to enable SPI4 clock"]
pub type SPI3_DMA_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_DMA_CLK_EN` writer - Set 1 to enable SPI4 clock"]
pub type SPI3_DMA_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 27>;
#[doc = "Field `APB_SARADC_CLK_EN` reader - Set 1 to enable APB_SARADC clock"]
pub type APB_SARADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `APB_SARADC_CLK_EN` writer - Set 1 to enable APB_SARADC clock"]
pub type APB_SARADC_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 28>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Set 1 to enable SYSTEMTIMER clock"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Set 1 to enable SYSTEMTIMER clock"]
pub type SYSTIMER_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 29>;
#[doc = "Field `ADC2_ARB_CLK_EN` reader - Set 1 to enable ADC2_ARB clock"]
pub type ADC2_ARB_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ARB_CLK_EN` writer - Set 1 to enable ADC2_ARB clock"]
pub type ADC2_ARB_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 30>;
#[doc = "Field `SPI4_CLK_EN` reader - Set 1 to enable SPI4 clock"]
pub type SPI4_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI4_CLK_EN` writer - Set 1 to enable SPI4 clock"]
pub type SPI4_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable TIMERS clock"]
    #[inline(always)]
    pub fn timers_clk_en(&self) -> TIMERS_CLK_EN_R {
        TIMERS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable SPI01 clock"]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable UART clock"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to enable WDG clock"]
    #[inline(always)]
    pub fn wdg_clk_en(&self) -> WDG_CLK_EN_R {
        WDG_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to enable I2S0 clock"]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to enable UART1 clock"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to enable SPI2 clock"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to enable I2C_EXT0 clock"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&self) -> I2C_EXT0_CLK_EN_R {
        I2C_EXT0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to enable UHCI0 clock"]
    #[inline(always)]
    pub fn uhci0_clk_en(&self) -> UHCI0_CLK_EN_R {
        UHCI0_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set 1 to enable RMT clock"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set 1 to enable PCNT clock"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set 1 to enable LEDC clock"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set 1 to enable UHCI1 clock"]
    #[inline(always)]
    pub fn uhci1_clk_en(&self) -> UHCI1_CLK_EN_R {
        UHCI1_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set 1 to enable TIMERGROUP clock"]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set 1 to enable EFUSE clock"]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set 1 to enable TIMERGROUP1 clock"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&self) -> TIMERGROUP1_CLK_EN_R {
        TIMERGROUP1_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set 1 to enable SPI3 clock"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set 1 to enable PWM0 clock"]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set 1 to enable I2C_EXT1 clock"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&self) -> I2C_EXT1_CLK_EN_R {
        I2C_EXT1_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set 1 to enable CAN clock"]
    #[inline(always)]
    pub fn can_clk_en(&self) -> CAN_CLK_EN_R {
        CAN_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set 1 to enable PWM1 clock"]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set 1 to enable I2S1 clock"]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set 1 to enable SPI2_DMA clock"]
    #[inline(always)]
    pub fn spi2_dma_clk_en(&self) -> SPI2_DMA_CLK_EN_R {
        SPI2_DMA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set 1 to enable USB clock"]
    #[inline(always)]
    pub fn usb_clk_en(&self) -> USB_CLK_EN_R {
        USB_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set 1 to enable UART_MEM clock"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set 1 to enable PWM2 clock"]
    #[inline(always)]
    pub fn pwm2_clk_en(&self) -> PWM2_CLK_EN_R {
        PWM2_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set 1 to enable PWM3 clock"]
    #[inline(always)]
    pub fn pwm3_clk_en(&self) -> PWM3_CLK_EN_R {
        PWM3_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set 1 to enable SPI4 clock"]
    #[inline(always)]
    pub fn spi3_dma_clk_en(&self) -> SPI3_DMA_CLK_EN_R {
        SPI3_DMA_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set 1 to enable APB_SARADC clock"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&self) -> APB_SARADC_CLK_EN_R {
        APB_SARADC_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set 1 to enable SYSTEMTIMER clock"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set 1 to enable ADC2_ARB clock"]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&self) -> ADC2_ARB_CLK_EN_R {
        ADC2_ARB_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set 1 to enable SPI4 clock"]
    #[inline(always)]
    pub fn spi4_clk_en(&self) -> SPI4_CLK_EN_R {
        SPI4_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable TIMERS clock"]
    #[inline(always)]
    pub fn timers_clk_en(&mut self) -> TIMERS_CLK_EN_W {
        TIMERS_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to enable SPI01 clock"]
    #[inline(always)]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W {
        SPI01_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable UART clock"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W {
        UART_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set 1 to enable WDG clock"]
    #[inline(always)]
    pub fn wdg_clk_en(&mut self) -> WDG_CLK_EN_W {
        WDG_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4 - Set 1 to enable I2S0 clock"]
    #[inline(always)]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W {
        I2S0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 5 - Set 1 to enable UART1 clock"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W {
        UART1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to enable SPI2 clock"]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W {
        SPI2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 7 - Set 1 to enable I2C_EXT0 clock"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&mut self) -> I2C_EXT0_CLK_EN_W {
        I2C_EXT0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 8 - Set 1 to enable UHCI0 clock"]
    #[inline(always)]
    pub fn uhci0_clk_en(&mut self) -> UHCI0_CLK_EN_W {
        UHCI0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - Set 1 to enable RMT clock"]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W {
        RMT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 10 - Set 1 to enable PCNT clock"]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W {
        PCNT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 11 - Set 1 to enable LEDC clock"]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W {
        LEDC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 12 - Set 1 to enable UHCI1 clock"]
    #[inline(always)]
    pub fn uhci1_clk_en(&mut self) -> UHCI1_CLK_EN_W {
        UHCI1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 13 - Set 1 to enable TIMERGROUP clock"]
    #[inline(always)]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W {
        TIMERGROUP_CLK_EN_W::new(self)
    }
    #[doc = "Bit 14 - Set 1 to enable EFUSE clock"]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W {
        EFUSE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 15 - Set 1 to enable TIMERGROUP1 clock"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&mut self) -> TIMERGROUP1_CLK_EN_W {
        TIMERGROUP1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 16 - Set 1 to enable SPI3 clock"]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W {
        SPI3_CLK_EN_W::new(self)
    }
    #[doc = "Bit 17 - Set 1 to enable PWM0 clock"]
    #[inline(always)]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W {
        PWM0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 18 - Set 1 to enable I2C_EXT1 clock"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&mut self) -> I2C_EXT1_CLK_EN_W {
        I2C_EXT1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 19 - Set 1 to enable CAN clock"]
    #[inline(always)]
    pub fn can_clk_en(&mut self) -> CAN_CLK_EN_W {
        CAN_CLK_EN_W::new(self)
    }
    #[doc = "Bit 20 - Set 1 to enable PWM1 clock"]
    #[inline(always)]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W {
        PWM1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 21 - Set 1 to enable I2S1 clock"]
    #[inline(always)]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W {
        I2S1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable SPI2_DMA clock"]
    #[inline(always)]
    pub fn spi2_dma_clk_en(&mut self) -> SPI2_DMA_CLK_EN_W {
        SPI2_DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 23 - Set 1 to enable USB clock"]
    #[inline(always)]
    pub fn usb_clk_en(&mut self) -> USB_CLK_EN_W {
        USB_CLK_EN_W::new(self)
    }
    #[doc = "Bit 24 - Set 1 to enable UART_MEM clock"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W {
        UART_MEM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Set 1 to enable PWM2 clock"]
    #[inline(always)]
    pub fn pwm2_clk_en(&mut self) -> PWM2_CLK_EN_W {
        PWM2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 26 - Set 1 to enable PWM3 clock"]
    #[inline(always)]
    pub fn pwm3_clk_en(&mut self) -> PWM3_CLK_EN_W {
        PWM3_CLK_EN_W::new(self)
    }
    #[doc = "Bit 27 - Set 1 to enable SPI4 clock"]
    #[inline(always)]
    pub fn spi3_dma_clk_en(&mut self) -> SPI3_DMA_CLK_EN_W {
        SPI3_DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 28 - Set 1 to enable APB_SARADC clock"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&mut self) -> APB_SARADC_CLK_EN_W {
        APB_SARADC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 29 - Set 1 to enable SYSTEMTIMER clock"]
    #[inline(always)]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W {
        SYSTIMER_CLK_EN_W::new(self)
    }
    #[doc = "Bit 30 - Set 1 to enable ADC2_ARB clock"]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&mut self) -> ADC2_ARB_CLK_EN_W {
        ADC2_ARB_CLK_EN_W::new(self)
    }
    #[doc = "Bit 31 - Set 1 to enable SPI4 clock"]
    #[inline(always)]
    pub fn spi4_clk_en(&mut self) -> SPI4_CLK_EN_W {
        SPI4_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral clock configuration regsiter 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en0](index.html) module"]
pub struct PERIP_CLK_EN0_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en0::R](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en0::W](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_CLK_EN0 to value 0xf9c1_e06f"]
impl crate::Resettable for PERIP_CLK_EN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf9c1_e06f
    }
}
