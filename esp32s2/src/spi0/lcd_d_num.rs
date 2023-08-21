#[doc = "Register `LCD_D_NUM` reader"]
pub type R = crate::R<LCD_D_NUM_SPEC>;
#[doc = "Register `LCD_D_NUM` writer"]
pub type W = crate::W<LCD_D_NUM_SPEC>;
#[doc = "Field `D_DQS_NUM` reader - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DQS_NUM_R = crate::FieldReader;
#[doc = "Field `D_DQS_NUM` writer - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DQS_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `D_CD_NUM` reader - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_CD_NUM_R = crate::FieldReader;
#[doc = "Field `D_CD_NUM` writer - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_CD_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `D_DE_NUM` reader - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DE_NUM_R = crate::FieldReader;
#[doc = "Field `D_DE_NUM` writer - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DE_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `D_HSYNC_NUM` reader - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_HSYNC_NUM_R = crate::FieldReader;
#[doc = "Field `D_HSYNC_NUM` writer - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_HSYNC_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `D_VSYNC_NUM` reader - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_VSYNC_NUM_R = crate::FieldReader;
#[doc = "Field `D_VSYNC_NUM` writer - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_VSYNC_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_num(&self) -> D_DQS_NUM_R {
        D_DQS_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_num(&self) -> D_CD_NUM_R {
        D_CD_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_num(&self) -> D_DE_NUM_R {
        D_DE_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_num(&self) -> D_HSYNC_NUM_R {
        D_HSYNC_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_num(&self) -> D_VSYNC_NUM_R {
        D_VSYNC_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_D_NUM")
            .field("d_dqs_num", &format_args!("{}", self.d_dqs_num().bits()))
            .field("d_cd_num", &format_args!("{}", self.d_cd_num().bits()))
            .field("d_de_num", &format_args!("{}", self.d_de_num().bits()))
            .field(
                "d_hsync_num",
                &format_args!("{}", self.d_hsync_num().bits()),
            )
            .field(
                "d_vsync_num",
                &format_args!("{}", self.d_vsync_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_D_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_dqs_num(&mut self) -> D_DQS_NUM_W<LCD_D_NUM_SPEC, 0> {
        D_DQS_NUM_W::new(self)
    }
    #[doc = "Bits 2:3 - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_cd_num(&mut self) -> D_CD_NUM_W<LCD_D_NUM_SPEC, 2> {
        D_CD_NUM_W::new(self)
    }
    #[doc = "Bits 4:5 - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_de_num(&mut self) -> D_DE_NUM_W<LCD_D_NUM_SPEC, 4> {
        D_DE_NUM_W::new(self)
    }
    #[doc = "Bits 6:7 - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_hsync_num(&mut self) -> D_HSYNC_NUM_W<LCD_D_NUM_SPEC, 6> {
        D_HSYNC_NUM_W::new(self)
    }
    #[doc = "Bits 8:9 - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_vsync_num(&mut self) -> D_VSYNC_NUM_W<LCD_D_NUM_SPEC, 8> {
        D_VSYNC_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LCD delay mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_d_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_d_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_D_NUM_SPEC;
impl crate::RegisterSpec for LCD_D_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_d_num::R`](R) reader structure"]
impl crate::Readable for LCD_D_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_d_num::W`](W) writer structure"]
impl crate::Writable for LCD_D_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_D_NUM to value 0"]
impl crate::Resettable for LCD_D_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
