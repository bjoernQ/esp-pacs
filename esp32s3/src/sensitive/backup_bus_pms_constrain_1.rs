#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART` reader - BackUp access uart permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART` writer - BackUp access uart permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 0>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1` reader - BackUp access g0spi_1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1` writer - BackUp access g0spi_1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0` reader - BackUp access g0spi_0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0` writer - BackUp access g0spi_0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 4>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_GPIO` reader - BackUp access gpio permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_GPIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_GPIO` writer - BackUp access gpio permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_GPIO_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 6>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE2` reader - BackUp access fe2 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE2` writer - BackUp access fe2 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE2_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE` reader - BackUp access fe permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE` writer - BackUp access fe permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 10>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTC` reader - BackUp access rtc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTC` writer - BackUp access rtc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTC_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 14>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_IO_MUX` reader - BackUp access io_mux permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_IO_MUX` writer - BackUp access io_mux permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 16>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_HINF` reader - BackUp access hinf permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_HINF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_HINF` writer - BackUp access hinf permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_HINF_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 20>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_MISC` reader - BackUp access misc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_MISC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_MISC` writer - BackUp access misc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_MISC_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 24>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C` reader - BackUp access i2c permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C` writer - BackUp access i2c permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 26>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S0` reader - BackUp access i2s0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2S0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S0` writer - BackUp access i2s0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2S0_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 28>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART1` reader - BackUp access uart1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART1` writer - BackUp access uart1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART1_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:1 - BackUp access uart permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UART_R {
        BACKUP_BUS_PMS_CONSTRAIN_UART_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BackUp access g0spi_1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - BackUp access g0spi_0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BackUp access gpio permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_gpio(&self) -> BACKUP_BUS_PMS_CONSTRAIN_GPIO_R {
        BACKUP_BUS_PMS_CONSTRAIN_GPIO_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - BackUp access fe2 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe2(&self) -> BACKUP_BUS_PMS_CONSTRAIN_FE2_R {
        BACKUP_BUS_PMS_CONSTRAIN_FE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BackUp access fe permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe(&self) -> BACKUP_BUS_PMS_CONSTRAIN_FE_R {
        BACKUP_BUS_PMS_CONSTRAIN_FE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BackUp access rtc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTC_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - BackUp access io_mux permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_io_mux(&self) -> BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R {
        BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - BackUp access hinf permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_hinf(&self) -> BACKUP_BUS_PMS_CONSTRAIN_HINF_R {
        BACKUP_BUS_PMS_CONSTRAIN_HINF_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - BackUp access misc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_misc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_MISC_R {
        BACKUP_BUS_PMS_CONSTRAIN_MISC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - BackUp access i2c permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BackUp access i2s0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2s0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2S0_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2S0_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BackUp access uart1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UART1_R {
        BACKUP_BUS_PMS_CONSTRAIN_UART1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BackUp access uart permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UART_W {
        BACKUP_BUS_PMS_CONSTRAIN_UART_W::new(self)
    }
    #[doc = "Bits 2:3 - BackUp access g0spi_1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W::new(self)
    }
    #[doc = "Bits 4:5 - BackUp access g0spi_0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W::new(self)
    }
    #[doc = "Bits 6:7 - BackUp access gpio permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_gpio(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_GPIO_W {
        BACKUP_BUS_PMS_CONSTRAIN_GPIO_W::new(self)
    }
    #[doc = "Bits 8:9 - BackUp access fe2 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe2(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_FE2_W {
        BACKUP_BUS_PMS_CONSTRAIN_FE2_W::new(self)
    }
    #[doc = "Bits 10:11 - BackUp access fe permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_FE_W {
        BACKUP_BUS_PMS_CONSTRAIN_FE_W::new(self)
    }
    #[doc = "Bits 14:15 - BackUp access rtc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_RTC_W {
        BACKUP_BUS_PMS_CONSTRAIN_RTC_W::new(self)
    }
    #[doc = "Bits 16:17 - BackUp access io_mux permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_io_mux(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W {
        BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W::new(self)
    }
    #[doc = "Bits 20:21 - BackUp access hinf permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_hinf(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_HINF_W {
        BACKUP_BUS_PMS_CONSTRAIN_HINF_W::new(self)
    }
    #[doc = "Bits 24:25 - BackUp access misc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_misc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_MISC_W {
        BACKUP_BUS_PMS_CONSTRAIN_MISC_W::new(self)
    }
    #[doc = "Bits 26:27 - BackUp access i2c permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_W {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_W::new(self)
    }
    #[doc = "Bits 28:29 - BackUp access i2s0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2s0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2S0_W {
        BACKUP_BUS_PMS_CONSTRAIN_I2S0_W::new(self)
    }
    #[doc = "Bits 30:31 - BackUp access uart1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UART1_W {
        BACKUP_BUS_PMS_CONSTRAIN_UART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_1](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_1 to value 0xff33_cfff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff33_cfff
    }
}
